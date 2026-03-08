//! Herramienta start-api en Rust (contrato tools).
//! Valida si el puerto está ocupado; opción fail/kill. Levanta la API, comprueba health (éxito = health 200).

use std::net::TcpListener;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;
use gesfer_tools::{FeedbackEntry, ToolResult, to_contract_json};
use serde::Deserialize;

const TOOL_ID: &str = "start-api";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PortBlocked {
    Fail,
    Kill,
}

#[derive(Parser)]
#[command(name = "start_api")]
struct Args {
    #[arg(long)]
    no_build: bool,
    #[arg(long)]
    profile: Option<String>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    config_path: Option<String>,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    output_json: bool,
    /// Si el puerto está ocupado: fail = error y salir; kill = cerrar proceso y continuar.
    #[arg(long, default_value = "fail", value_parser = ["fail", "kill"])]
    port_blocked: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    api_working_dir: String,
    default_profile: Option<String>,
    default_port: Option<u16>,
    health_url: Option<String>,
    health_check_timeout_seconds: Option<u64>,
}

fn main() {
    let start = Instant::now();
    let mut feedback = Vec::new();

    let args = Args::parse();
    let port_blocked = if args.port_blocked.to_lowercase() == "kill" {
        PortBlocked::Kill
    } else {
        PortBlocked::Fail
    };

    feedback.push(FeedbackEntry::info("init", "Iniciando start-api (Rust)"));

    let repo_root = std::env::var("GESFER_REPO_ROOT")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().display().to_string());
    let config_path = args.config_path.as_deref().unwrap_or("start-api-config.json");
    let config = load_config(&repo_root, config_path, &mut feedback);
    let config = match config {
        Some(c) => c,
        None => {
                emit_result(false, 1, "Config no encontrado o inválido", feedback, None, start, &args);
            std::process::exit(1);
        }
    };

    let port = args.port.unwrap_or(config.default_port.unwrap_or(5010));
    let health_url = config
        .health_url
        .clone()
        .unwrap_or_else(|| format!("http://127.0.0.1:{}/health", port));
    let profile = args
        .profile
        .clone()
        .or(config.default_profile.clone())
        .unwrap_or_else(|| "Development".to_string());

    // 1. Comprobar si el puerto está ocupado
    feedback.push(FeedbackEntry::info(
        "port-check",
        &format!("Comprobando puerto {}...", port),
    ));
    if port_in_use(port) {
        feedback.push(FeedbackEntry::warning(
            "port-check",
            &format!("El puerto {} está ocupado.", port),
            None,
        ));
        match port_blocked {
            PortBlocked::Fail => {
                feedback.push(FeedbackEntry::error(
                    "port-check",
                    "Puerto ocupado. Use --port-blocked kill para liberar el proceso o cambie el puerto.",
                    None,
                ));
                let data = serde_json::json!({ "port": port, "port_in_use": true });
                emit_result(false, 2, "Puerto ocupado", feedback, Some(data), start, &args);
                std::process::exit(2);
            }
            PortBlocked::Kill => {
                feedback.push(FeedbackEntry::info("port-kill", "Intentando cerrar proceso que usa el puerto..."));
                if let Err(e) = kill_process_on_port(port) {
                    feedback.push(FeedbackEntry::error(
                        "port-kill",
                        "No se pudo liberar el puerto",
                        Some(&e),
                    ));
                    let data = serde_json::json!({ "port": port, "port_in_use": true });
                    emit_result(false, 3, "No se pudo liberar el puerto", feedback, Some(data), start, &args);
                    std::process::exit(3);
                }
                feedback.push(FeedbackEntry::info("port-kill", "Puerto liberado."));
                thread::sleep(Duration::from_secs(1));
                if port_in_use(port) {
                    feedback.push(FeedbackEntry::error(
                        "port-check",
                        "El puerto sigue ocupado tras intentar cerrar el proceso.",
                        None,
                    ));
                    emit_result(false, 3, "Puerto aún ocupado", feedback, None, start, &args);
                    std::process::exit(3);
                }
            }
        }
    } else {
        feedback.push(FeedbackEntry::info("port-check", "Puerto libre."));
    }

    let api_dir = std::path::Path::new(&repo_root).join(&config.api_working_dir);
    if !api_dir.exists() {
        feedback.push(FeedbackEntry::error(
            "init",
            &format!("Directorio API no encontrado: {}", api_dir.display()),
            None,
        ));
        emit_result(false, 4, "Directorio API no encontrado", feedback, None, start, &args);
        std::process::exit(4);
    }

    // 2. Build opcional
    if !args.no_build {
        feedback.push(FeedbackEntry::info("build", "Compilando proyecto..."));
        let build_start = Instant::now();
        let out = Command::new("dotnet")
            .args(["build", &config.api_working_dir, "-c", "Release"])
            .current_dir(&repo_root)
            .output();
        let build_ms = build_start.elapsed().as_millis() as u64;
        match out {
            Ok(o) if o.status.success() => {
                feedback.push(FeedbackEntry::info("build", "Build OK"));
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                feedback.push(FeedbackEntry::error("build", "Build fallido", Some(stderr.as_ref())));
                emit_result(false, 5, "Build fallido", feedback, None, start, &args);
                std::process::exit(5);
            }
            Err(e) => {
                feedback.push(FeedbackEntry::error("build", "No se pudo ejecutar dotnet build", Some(&e.to_string())));
                emit_result(false, 5, "dotnet build no disponible", feedback, None, start, &args);
                std::process::exit(5);
            }
        }
    } else {
        feedback.push(FeedbackEntry::info("build", "NoBuild: omitiendo compilación"));
    }

    // 3. Lanzar API en segundo plano
    feedback.push(FeedbackEntry::info(
        "launch",
        &format!("Levantando API en {} (Profile: {}, Port: {})", api_dir.display(), profile, port),
    ));
    let mut child = match Command::new("dotnet")
        .args(["run", "--no-build", "-c", "Release"])
        .current_dir(&api_dir)
        .env("ASPNETCORE_URLS", format!("http://127.0.0.1:{}", port))
        .env("ASPNETCORE_ENVIRONMENT", &profile)
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            feedback.push(FeedbackEntry::error("launch", "No se pudo ejecutar dotnet run", Some(&e.to_string())));
            emit_result(false, 6, "Error al lanzar API", feedback, None, start, &args);
            std::process::exit(6);
        }
    };
    let pid = child.id();
    feedback.push(FeedbackEntry::info("launch", &format!("API iniciada con PID {}", pid)));

    // Asegurar que al salir (éxito o panic) no dejamos el proceso huérfano sin avisar; el contrato es "levantar" y devolver resultado, el proceso sigue en background.
    let _ = child.stdin.take();

    let timeout_secs = config.health_check_timeout_seconds.unwrap_or(30);
    let step = Duration::from_secs(2);
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let mut healthy = false;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());

    while Instant::now() < deadline {
        thread::sleep(step);
        if let Ok(resp) = client.get(&health_url).send() {
            if resp.status().as_u16() == 200 {
                healthy = true;
                feedback.push(FeedbackEntry::info("healthcheck", &format!("Health OK: {}", health_url)));
                break;
            }
        }
        feedback.push(FeedbackEntry::info(
            "healthcheck",
            &format!("Esperando salud ({}/{} s)...", Instant::now().duration_since(start).as_secs(), timeout_secs),
        ));
    }

    if !healthy {
        feedback.push(FeedbackEntry::warning(
            "healthcheck",
            &format!("Timeout salud; API arrancada (PID {}). Compruebe {}", pid, health_url),
            None,
        ));
        let data = serde_json::json!({
            "url_base": health_url,
            "port": port,
            "profile": profile,
            "pid": pid,
            "healthy": false
        });
        emit_result(false, 7, "Health no respondió a tiempo", feedback, Some(data), start, &args);
        std::process::exit(7);
    }

    let data = serde_json::json!({
        "url_base": health_url,
        "port": port,
        "profile": profile,
        "pid": pid,
        "healthy": true
    });
    feedback.push(FeedbackEntry::info("done", &format!("API levantada. PID: {} URL: {}", pid, health_url)));
    emit_result(true, 0, "API levantada; health OK", feedback, Some(data), start, &args);
    std::process::exit(0);
}

fn load_config(repo_root: &str, config_path: &str, feedback: &mut Vec<FeedbackEntry>) -> Option<Config> {
    let mut path = std::path::PathBuf::from(repo_root);
    path.push(config_path);
    if !path.exists() {
        path = std::path::PathBuf::from(repo_root).join("scripts").join("tools").join("start-api").join(config_path);
    }
    let s = std::fs::read_to_string(&path).ok()?;
    let mut raw: serde_json::Value = serde_json::from_str(&s).ok()?;
    let obj = raw.as_object_mut()?;
    let api_working_dir = obj.get("apiWorkingDir")?.as_str()?.to_string();
    let default_profile = obj.get("defaultProfile").and_then(|v| v.as_str()).map(String::from);
    let default_port = obj.get("defaultPort").and_then(|v| v.as_u64()).map(|n| n as u16);
    let health_url = obj.get("healthUrl").and_then(|v| v.as_str()).map(String::from);
    let health_check_timeout_seconds = obj.get("healthCheckTimeoutSeconds").and_then(|v| v.as_u64());
    Some(Config {
        api_working_dir,
        default_profile,
        default_port,
        health_url,
        health_check_timeout_seconds,
    })
}

fn port_in_use(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_err()
}

/// En Windows: netstat -ano, parsear líneas con :port y LISTENING, obtener PID; taskkill /PID x /F.
#[cfg(target_os = "windows")]
fn kill_process_on_port(port: u16) -> Result<(), String> {
    let port_str = format!(":{}", port);
    let out = Command::new("netstat")
        .args(["-ano"])
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let pids: Vec<u32> = stdout
        .lines()
        .filter(|line| line.contains("LISTENING") && line.contains(&port_str))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts.last().and_then(|s| s.parse::<u32>().ok())
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    if pids.is_empty() {
        return Err("No se encontró proceso escuchando en el puerto".to_string());
    }
    for pid in pids {
        let _ = Command::new("taskkill").args(["/PID", &pid.to_string(), "/F"]).output();
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn kill_process_on_port(_port: u16) -> Result<(), String> {
    Err("Liberar puerto (kill) solo soportado en Windows".to_string())
}

fn emit_result(
    success: bool,
    exit_code: i32,
    message: &str,
    feedback: Vec<FeedbackEntry>,
    data: Option<serde_json::Value>,
    start: Instant,
    args: &Args,
) {
    let duration_ms = start.elapsed().as_millis() as u64;
    let result = ToolResult {
        tool_id: TOOL_ID.to_string(),
        exit_code,
        success,
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: message.to_string(),
        feedback,
        data,
        duration_ms: Some(duration_ms),
    };
    let json = to_contract_json(&result).expect("serialize");
    if args.output_json || std::env::var("TOOLS_OUTPUT_JSON").as_deref() == Ok("1") {
        println!("{}", json);
    }
    if let Some(ref path) = args.output_path {
        let _ = std::fs::write(path, &json);
    }
}
