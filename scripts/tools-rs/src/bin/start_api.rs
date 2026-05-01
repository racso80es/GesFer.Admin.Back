//! start-api — contrato SddIA/tools/start-api/spec.md (puerto, build, health, exit 0–8).

use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;
use clap::ValueEnum;
use gesfer_tools::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

const TOOL_ID: &str = "start-api";
const SLN_REL: &str = "src/GesFer.Admin.Back.sln";
const MAX_CAPTURE_BYTES: usize = 65_536;
const HEALTH_POLL_MS: u64 = 500;

const MYSQL_MARKERS: &[&str] = &[
    "Unable to connect to any of the specified MySQL hosts",
    "MySqlConnector.MySqlException",
];

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PortBlockedArg {
    Fail,
    Kill,
}

#[derive(Debug, Parser)]
#[command(name = "start_api")]
struct Cli {
    #[arg(long)]
    config_path: Option<String>,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_build: bool,
    #[arg(long)]
    profile: Option<String>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long, value_enum)]
    port_blocked: Option<PortBlockedArg>,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    output_json: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct RequestOverlay {
    #[serde(default, alias = "NoBuild", alias = "no_build")]
    no_build: Option<bool>,
    #[serde(default, alias = "Profile")]
    profile: Option<String>,
    #[serde(default, alias = "Port")]
    port: Option<u16>,
    #[serde(default, alias = "ConfigPath", alias = "config_path")]
    config_path: Option<String>,
    #[serde(default, alias = "OutputPath", alias = "output_path")]
    output_path: Option<String>,
    #[serde(default, alias = "OutputJson", alias = "output_json")]
    output_json: Option<bool>,
    #[serde(default, alias = "PortBlocked", alias = "port_blocked")]
    port_blocked: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartApiConfig {
    api_working_dir: String,
    #[serde(default)]
    default_profile: Option<String>,
    #[serde(default)]
    default_port: Option<u16>,
    health_url: String,
    #[serde(default)]
    health_check_timeout_seconds: Option<u64>,
    #[serde(default)]
    port_blocked: Option<String>,
}

fn repo_root() -> PathBuf {
    std::env::var("GESFER_REPO_ROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn resolve_path(root: &Path, p: &str) -> PathBuf {
    let p = PathBuf::from(p);
    if p.is_absolute() {
        p
    } else {
        root.join(p)
    }
}

fn default_config_path() -> PathBuf {
    let beside_exe = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(|p| p.join("start-api-config.json")));
    if let Some(ref p) = beside_exe {
        if p.is_file() {
            return p.clone();
        }
    }
    if let Ok(root) = std::env::var("GESFER_REPO_ROOT") {
        let p = PathBuf::from(root).join("scripts/tools/start-api/start-api-config.json");
        if p.is_file() {
            return p;
        }
    }
    beside_exe.unwrap_or_else(|| PathBuf::from("start-api-config.json"))
}

fn parse_port_blocked(s: Option<&str>) -> PortBlockedArg {
    match s.map(|x| x.trim().to_ascii_lowercase()).as_deref() {
        Some("kill") => PortBlockedArg::Kill,
        _ => PortBlockedArg::Fail,
    }
}

fn port_in_use(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_err()
}

fn local_addr_uses_port(addr: &str, port: u16) -> bool {
    addr.rsplit_once(':').map(|(_, p)| p == port.to_string()).unwrap_or(false)
}

fn windows_pids_listening_on_port(port: u16) -> Vec<u32> {
    let Ok(out) = Command::new("cmd")
        .args(["/C", "netstat -ano"])
        .output()
    else {
        return vec![];
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let mut pids = vec![];
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 || parts[3] != "LISTENING" {
            continue;
        }
        if !local_addr_uses_port(parts[1], port) {
            continue;
        }
        if let Ok(pid) = parts[4].parse::<u32>() {
            pids.push(pid);
        }
    }
    pids.sort_unstable();
    pids.dedup();
    pids
}

fn windows_kill_pids(pids: &[u32]) {
    for pid in pids {
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output();
    }
}

fn try_free_port_windows(port: u16) {
    for _ in 0..5 {
        let pids = windows_pids_listening_on_port(port);
        if pids.is_empty() {
            break;
        }
        windows_kill_pids(&pids);
        thread::sleep(Duration::from_millis(500));
    }
}

fn effective_health_url(cfg: &StartApiConfig, port_override: Option<u16>) -> Result<String, String> {
    if let Some(p) = port_override {
        let mut u = Url::parse(&cfg.health_url).map_err(|e| format!("healthUrl inválida: {}", e))?;
        u.set_host(Some("localhost"))
            .map_err(|_| "no se pudo fijar host localhost".to_string())?;
        u.set_port(Some(p))
            .map_err(|_| "no se pudo fijar puerto".to_string())?;
        return Ok(u.to_string());
    }
    Ok(cfg.health_url.clone())
}

fn port_from_health_url(health_url: &str) -> Result<u16, String> {
    let u = Url::parse(health_url).map_err(|e| format!("{}", e))?;
    u.port_or_known_default()
        .map(|p| p as u16)
        .ok_or_else(|| "URL sin puerto".to_string())
}

fn dotnet_build_sln(root: &Path) -> Result<(), String> {
    let building = Arc::new(AtomicBool::new(true));
    let b = building.clone();
    let hb = thread::spawn(move || {
        while b.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(10));
            eprintln!("[start-api] compilación en curso…");
        }
    });
    let sln = root.join(SLN_REL);
    let out = Command::new("dotnet")
        .current_dir(root)
        .args(["build", sln.to_str().unwrap_or(SLN_REL)])
        .output();
    building.store(false, Ordering::Relaxed);
    let _ = hb.join();
    match out {
        Ok(o) if o.status.success() => Ok(()),
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).trim().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

struct CaptureState {
    buf: Mutex<String>,
}

impl CaptureState {
    fn new() -> Self {
        Self {
            buf: Mutex::new(String::new()),
        }
    }

    fn push(&self, chunk: &str) {
        let mut g = self.buf.lock().unwrap();
        g.push_str(chunk);
        if g.len() > MAX_CAPTURE_BYTES {
            let trim = g.len() - MAX_CAPTURE_BYTES;
            *g = g[trim..].to_string();
        }
    }

    fn snapshot(&self) -> String {
        self.buf.lock().unwrap().clone()
    }
}

fn spawn_log_reader<R: Read + Send + 'static>(read: R, cap: Arc<CaptureState>) {
    thread::spawn(move || {
        let mut br = BufReader::new(read);
        let mut line = String::new();
        loop {
            line.clear();
            match br.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => cap.push(&line),
                Err(_) => break,
            }
        }
    });
}

fn child_output_contains_db_error(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    MYSQL_MARKERS
        .iter()
        .any(|m| lower.contains(&m.to_ascii_lowercase()))
}

fn wait_for_health(
    client: &Client,
    health_url: &str,
    timeout_secs: u64,
    child_capture: &CaptureState,
) -> Result<(), (i32, String, serde_json::Value)> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs.max(1));
    let started = Instant::now();
    while Instant::now() < deadline {
        if child_output_contains_db_error(&child_capture.snapshot()) {
            return Err((
                8,
                "Base de datos (MySQL) no disponible".to_string(),
                serde_json::json!({
                    "error_type": "database_unavailable",
                    "health_wait_elapsed_secs": started.elapsed().as_secs(),
                    "health_timeout_config_secs": timeout_secs
                }),
            ));
        }
        match client.get(health_url).timeout(Duration::from_secs(3)).send() {
            Ok(r) if r.status() == 200 => return Ok(()),
            _ => thread::sleep(Duration::from_millis(HEALTH_POLL_MS)),
        }
    }
    Err((
        7,
        "Health no respondió a tiempo".to_string(),
        serde_json::json!({
            "error_type": "health_timeout",
            "health_wait_elapsed_secs": started.elapsed().as_secs(),
            "health_timeout_config_secs": timeout_secs
        }),
    ))
}

fn finish(
    res: CapsuleResponse,
    output_path: Option<&str>,
    emit_stdout_json: bool,
) -> ! {
    if let Some(p) = output_path {
        let _ = std::fs::write(
            p,
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string()),
        );
    }
    if emit_stdout_json {
        let _ = write_capsule_response(&res);
    }
    std::process::exit(res.exit_code);
}

fn main() {
    let started = Instant::now();
    let mut feedback: Vec<FeedbackEntry> = vec![FeedbackEntry::info("init", "Iniciando start-api (Rust)")];

    let capsule_req = match try_read_capsule_request() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut cli = Cli::parse();
    let overlay: RequestOverlay = capsule_req
        .as_ref()
        .and_then(|r| serde_json::from_value(r.request.clone()).ok())
        .unwrap_or_default();

    if cli.config_path.is_none() {
        if let Some(ref p) = overlay.config_path {
            cli.config_path = Some(p.clone());
        }
    }
    let no_build = cli.no_build || overlay.no_build.unwrap_or(false);
    if cli.profile.is_none() {
        cli.profile = overlay.profile.clone();
    }
    if cli.port.is_none() {
        cli.port = overlay.port;
    }
    if cli.output_path.is_none() {
        cli.output_path = overlay.output_path.clone();
    }
    if !cli.output_json {
        if let Some(true) = overlay.output_json {
            cli.output_json = true;
        }
    }
    if cli.port_blocked.is_none() {
        if let Some(ref pb) = overlay.port_blocked {
            cli.port_blocked = Some(parse_port_blocked(Some(pb.as_str())));
        }
    }

    let emit_json = capsule_req.is_some() || cli.output_json;

    let root = repo_root();
    let cfg_path = cli
        .config_path
        .as_deref()
        .map(|p| resolve_path(&root, p))
        .unwrap_or_else(default_config_path);

    let cfg_raw = match std::fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(e) => {
            feedback.push(FeedbackEntry::error("error", "Config no encontrado o ilegible", Some(&e.to_string())));
            finish(
                CapsuleResponse::tool(
                    TOOL_ID,
                    false,
                    1,
                    "Config no encontrado o inválido",
                    feedback,
                    serde_json::json!({ "configPath": cfg_path.display().to_string() }),
                    Some(started.elapsed().as_millis() as u64),
                ),
                cli.output_path.as_deref(),
                emit_json,
            );
        }
    };

    let cfg: StartApiConfig = match serde_json::from_str(&cfg_raw) {
        Ok(c) => c,
        Err(e) => {
            feedback.push(FeedbackEntry::error("error", "JSON de configuración inválido", Some(&e.to_string())));
            finish(
                CapsuleResponse::tool(
                    TOOL_ID,
                    false,
                    1,
                    "Config no encontrado o inválido",
                    feedback,
                    serde_json::json!({ "configPath": cfg_path.display().to_string() }),
                    Some(started.elapsed().as_millis() as u64),
                ),
                cli.output_path.as_deref(),
                emit_json,
            );
        }
    };

    let port_blocked = cli
        .port_blocked
        .unwrap_or_else(|| parse_port_blocked(cfg.port_blocked.as_deref()));

    let profile = cli
        .profile
        .clone()
        .or_else(|| cfg.default_profile.clone())
        .unwrap_or_else(|| "Development".to_string());

    let health_url = match effective_health_url(&cfg, cli.port) {
        Ok(u) => u,
        Err(msg) => {
            feedback.push(FeedbackEntry::error("error", &msg, None));
            finish(
                CapsuleResponse::tool(
                    TOOL_ID,
                    false,
                    1,
                    &msg,
                    feedback,
                    serde_json::json!({}),
                    Some(started.elapsed().as_millis() as u64),
                ),
                cli.output_path.as_deref(),
                emit_json,
            );
        }
    };

    let check_port = cli.port.or_else(|| {
        port_from_health_url(&health_url)
            .ok()
            .or(cfg.default_port)
    });

    let Some(check_port) = check_port else {
        feedback.push(FeedbackEntry::error("error", "No se pudo determinar puerto para comprobación", None));
        finish(
            CapsuleResponse::tool(
                TOOL_ID,
                false,
                1,
                "Config inválida: sin puerto",
                feedback,
                serde_json::json!({}),
                Some(started.elapsed().as_millis() as u64),
            ),
            cli.output_path.as_deref(),
            emit_json,
        );
    };

    feedback.push(FeedbackEntry::info("port-check", &format!("Comprobando puerto {}", check_port)));
    if port_in_use(check_port) {
        match port_blocked {
            PortBlockedArg::Fail => {
                feedback.push(FeedbackEntry::error(
                    "port-check",
                    "Puerto ocupado (port-blocked=fail)",
                    None,
                ));
                finish(
                    CapsuleResponse::tool(
                        TOOL_ID,
                        false,
                        2,
                        "Puerto ocupado",
                        feedback,
                        serde_json::json!({ "port": check_port }),
                        Some(started.elapsed().as_millis() as u64),
                    ),
                    cli.output_path.as_deref(),
                    emit_json,
                );
            }
            PortBlockedArg::Kill => {
                feedback.push(FeedbackEntry::warning(
                    "port-kill",
                    "Puerto ocupado; intentando liberar (kill)",
                    None,
                ));
                try_free_port_windows(check_port);
                thread::sleep(Duration::from_millis(300));
                if port_in_use(check_port) {
                    feedback.push(FeedbackEntry::error(
                        "port-kill",
                        "Puerto sigue ocupado tras kill",
                        None,
                    ));
                    finish(
                        CapsuleResponse::tool(
                            TOOL_ID,
                            false,
                            3,
                            "Puerto no liberado",
                            feedback,
                            serde_json::json!({ "port": check_port }),
                            Some(started.elapsed().as_millis() as u64),
                        ),
                        cli.output_path.as_deref(),
                        emit_json,
                    );
                }
            }
        }
    }

    let api_dir = resolve_path(&root, &cfg.api_working_dir);
    if !api_dir.is_dir() {
        feedback.push(FeedbackEntry::error(
            "error",
            "Directorio API no encontrado",
            Some(&api_dir.display().to_string()),
        ));
        finish(
            CapsuleResponse::tool(
                TOOL_ID,
                false,
                4,
                "Directorio API no encontrado",
                feedback,
                serde_json::json!({ "apiWorkingDir": api_dir.display().to_string() }),
                Some(started.elapsed().as_millis() as u64),
            ),
            cli.output_path.as_deref(),
            emit_json,
        );
    }

    if !no_build {
        feedback.push(FeedbackEntry::info("build", "dotnet build solución"));
        if let Err(err) = dotnet_build_sln(&root) {
            feedback.push(FeedbackEntry::error("build", "Build fallido", Some(&err)));
            finish(
                CapsuleResponse::tool(
                    TOOL_ID,
                    false,
                    5,
                    "Build fallido",
                    feedback,
                    serde_json::json!({
                        "error_type": "build_failed",
                        "detail": err
                    }),
                    Some(started.elapsed().as_millis() as u64),
                ),
                cli.output_path.as_deref(),
                emit_json,
            );
        }
    }

    let mut args: Vec<String> = vec!["run".into(), "--launch-profile".into(), profile.clone()];
    if no_build {
        args.push("--no-build".into());
    }

    feedback.push(FeedbackEntry::info(
        "launch",
        &format!("dotnet {} (wd: {})", args.join(" "), api_dir.display()),
    ));

    let capture = Arc::new(CaptureState::new());
    let mut cmd = Command::new("dotnet");
    cmd.current_dir(&api_dir)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child: Child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            feedback.push(FeedbackEntry::error("launch", "No se pudo iniciar dotnet run", Some(&e.to_string())));
            finish(
                CapsuleResponse::tool(
                    TOOL_ID,
                    false,
                    6,
                    "Error al lanzar dotnet run",
                    feedback,
                    serde_json::json!({ "detail": e.to_string() }),
                    Some(started.elapsed().as_millis() as u64),
                ),
                cli.output_path.as_deref(),
                emit_json,
            );
        }
    };

    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");
    let cap_out = capture.clone();
    let cap_err = capture.clone();
    spawn_log_reader(stdout, cap_out);
    spawn_log_reader(stderr, cap_err);

    let pid = child.id();
    let timeout_secs = cfg.health_check_timeout_seconds.unwrap_or(30);
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_else(|_| Client::new());

    let health_outcome = wait_for_health(&client, &health_url, timeout_secs, &capture);
    if let Err((code, msg, extra)) = health_outcome {
        let _ = child.kill();
        let _ = child.wait();
        feedback.push(FeedbackEntry::error("healthcheck", &msg, None));
        let mut result = extra;
        if let Some(obj) = result.as_object_mut() {
            obj.insert("pid".to_string(), serde_json::json!(pid));
            obj.insert("port".to_string(), serde_json::json!(check_port));
        }
        finish(
            CapsuleResponse::tool(
                TOOL_ID,
                false,
                code,
                &msg,
                feedback,
                result,
                Some(started.elapsed().as_millis() as u64),
            ),
            cli.output_path.as_deref(),
            emit_json,
        );
    }

    let url_base = health_url
        .trim_end_matches('/')
        .rsplit_once('/')
        .map(|(b, _)| format!("{}/", b.trim_end_matches('/')))
        .unwrap_or_else(|| health_url.clone());

    feedback.push(FeedbackEntry::info("healthcheck", "Health OK (HTTP 200)"));
    feedback.push(FeedbackEntry::info("done", "API en ejecución"));

    finish(
        CapsuleResponse::tool(
            TOOL_ID,
            true,
            0,
            "API lista (health 200)",
            feedback,
            serde_json::json!({
                "url_base": url_base,
                "health_url": health_url,
                "port": check_port,
                "pid": pid,
                "healthy": true,
                "profile": profile
            }),
            Some(started.elapsed().as_millis() as u64),
        ),
        cli.output_path.as_deref(),
        emit_json,
    );
}
