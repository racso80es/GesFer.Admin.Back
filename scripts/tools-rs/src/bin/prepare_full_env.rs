use anyhow::{anyhow, Context, Result};
use clap::Parser;
use gesfer_capsule::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrepareEnvConfig {
    #[serde(default)]
    docker_compose_path: String,
    #[serde(default)]
    mysql_container_name: String,
    #[serde(default)]
    docker_services: Vec<String>,
    #[serde(default)]
    start_clients: Vec<ClientConfig>,
    #[serde(default)]
    health_check: HealthCheck,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientConfig {
    name: String,
    working_dir: String,
    command: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HealthCheck {
    #[serde(default = "default_mysql_max_attempts")]
    mysql_max_attempts: u32,
    #[serde(default = "default_mysql_retry_seconds")]
    mysql_retry_seconds: u64,
}

fn default_mysql_max_attempts() -> u32 {
    30
}
fn default_mysql_retry_seconds() -> u64 {
    2
}

#[derive(Parser, Debug)]
#[command(name = "prepare_full_env", disable_help_subcommand = true)]
struct Cli {
    #[arg(long)]
    docker_only: bool,
    #[arg(long)]
    no_docker: bool,
    #[arg(long)]
    output_json: bool,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    config_path: Option<String>,
}

#[derive(Debug, Clone)]
struct EffectiveRequest {
    docker_only: bool,
    no_docker: bool,
    output_json: bool,
    output_path: Option<String>,
    config_path: Option<String>,
}

fn normalize_legacy_args(mut args: Vec<String>) -> Vec<String> {
    // Soportar invocación humana heredada (PowerShell-like) usada por el .bat: -DockerOnly, -NoDocker, -ConfigPath, ...
    // Se traduce a flags clap: --docker-only, --no-docker, --config-path, ...
    for a in &mut args {
        let lower = a.to_ascii_lowercase();
        match lower.as_str() {
            "-dockeronly" => *a = "--docker-only".to_string(),
            "-nodocker" => *a = "--no-docker".to_string(),
            "-outputjson" => *a = "--output-json".to_string(),
            "-outputpath" => *a = "--output-path".to_string(),
            "-configpath" => *a = "--config-path".to_string(),
            _ => {}
        }
    }
    args
}

fn load_config(config_path: Option<&str>) -> Result<(PrepareEnvConfig, String)> {
    let path = resolve_config_path(config_path)?;
    let raw = fs::read_to_string(&path).with_context(|| format!("No se pudo leer config: {}", path))?;
    let cfg: PrepareEnvConfig =
        serde_json::from_str(&raw).with_context(|| format!("JSON inválido en config: {}", path))?;

    Ok((cfg, path))
}

fn resolve_config_path(config_path: Option<&str>) -> Result<String> {
    if let Some(p) = config_path {
        return Ok(p.to_string());
    }

    // 1) Relativo al repo root (cwd): scripts/tools/prepare-full-env/prepare-env.json
    let p1 = PathBuf::from("scripts/tools/prepare-full-env/prepare-env.json");
    if p1.exists() {
        return Ok(p1.to_string_lossy().to_string());
    }

    // 2) Junto al exe (cuando se invoca desde la cápsula)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p2 = dir.join("prepare-env.json");
            if p2.exists() {
                return Ok(p2.to_string_lossy().to_string());
            }
        }
    }

    Err(anyhow!(
        "No se encontró config por defecto. Usa --config-path o coloca prepare-env.json en la cápsula."
    ))
}

fn run_cmd(mut cmd: Command) -> Result<(i32, String)> {
    let out = cmd.output().context("Fallo al ejecutar comando")?;
    let code = out.status.code().unwrap_or(1);
    let mut text = String::new();
    if !out.stdout.is_empty() {
        text.push_str(&String::from_utf8_lossy(&out.stdout));
    }
    if !out.stderr.is_empty() {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&String::from_utf8_lossy(&out.stderr));
    }
    Ok((code, text.trim().to_string()))
}

fn docker_compose_up(compose_path: &str, services: &[String]) -> Result<()> {
    let mut cmd = Command::new("docker");
    cmd.arg("compose").arg("-f").arg(compose_path).arg("up").arg("-d");
    for s in services {
        cmd.arg(s);
    }
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("docker compose up falló: {}", text));
    }
    Ok(())
}

fn docker_health_status(container: &str) -> Result<String> {
    let mut cmd = Command::new("docker");
    cmd.arg("inspect")
        .arg("-f")
        .arg("{{.State.Health.Status}}")
        .arg(container);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        // Si no hay healthcheck, docker devuelve error: tratamos como "unknown".
        return Ok("unknown".to_string());
    }
    Ok(text)
}

fn docker_is_running(container: &str) -> Result<bool> {
    let mut cmd = Command::new("docker");
    cmd.arg("inspect")
        .arg("-f")
        .arg("{{.State.Running}}")
        .arg(container);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Ok(false);
    }
    Ok(text.trim().eq_ignore_ascii_case("true"))
}

fn wait_mysql_ready(container: &str, hc: &HealthCheck) -> Result<()> {
    let delay = Duration::from_secs(hc.mysql_retry_seconds);
    for _ in 0..hc.mysql_max_attempts {
        // Preferir healthcheck si existe.
        let health = docker_health_status(container)?;
        if health.eq_ignore_ascii_case("healthy") {
            return Ok(());
        }
        if health.eq_ignore_ascii_case("unknown") {
            if docker_is_running(container)? {
                return Ok(());
            }
        }
        std::thread::sleep(delay);
    }
    Err(anyhow!("MySQL no estuvo listo a tiempo (container={})", container))
}

fn start_clients(clients: &[ClientConfig]) -> Result<Vec<serde_json::Value>> {
    let mut started = Vec::new();
    for c in clients {
        // Nota: no mantenemos el proceso vivo/gestionado; lanzamos y devolvemos el PID si aplica.
        let mut cmd = if cfg!(windows) {
            let mut ps = Command::new("powershell");
            ps.arg("-NoProfile").arg("-Command");
            ps
        } else {
            let mut sh = Command::new("sh");
            sh.arg("-lc");
            sh
        };
        let wd = Path::new(&c.working_dir);
        cmd.current_dir(wd);
        cmd.arg(&c.command);
        let child = cmd.spawn().with_context(|| format!("No se pudo iniciar cliente {}", c.name))?;
        started.push(json!({
            "name": c.name,
            "workingDir": c.working_dir,
            "command": c.command,
            "pid": child.id()
        }));
    }
    Ok(started)
}

fn main() {
    let started_at = Instant::now();
    let tool_id = "prepare-full-env";
    let mut feedback: Vec<FeedbackEntry> = vec![FeedbackEntry::info("init", "Inicio prepare-full-env")];

    let mut effective_request: Option<EffectiveRequest> = None;
    let mut force_stdout = false;

    let run = (|| -> Result<serde_json::Value> {
        let effective = match try_read_capsule_request() {
            Ok(Some(req)) => {
                // En modo cápsula (agente), siempre emitimos stdout (contrato v2).
                force_stdout = true;
                // request: { DockerOnly?, NoDocker?, OutputJson?, OutputPath?, ConfigPath? } (case-insensitive por serde Value)
                let docker_only = req.request.get("DockerOnly").and_then(|v| v.as_bool()).unwrap_or(false)
                    || req.request.get("dockerOnly").and_then(|v| v.as_bool()).unwrap_or(false);
                let no_docker = req.request.get("NoDocker").and_then(|v| v.as_bool()).unwrap_or(false)
                    || req.request.get("noDocker").and_then(|v| v.as_bool()).unwrap_or(false);
                let output_json = req.request.get("OutputJson").and_then(|v| v.as_bool()).unwrap_or(false)
                    || req.request.get("outputJson").and_then(|v| v.as_bool()).unwrap_or(false);
                let output_path = req
                    .request
                    .get("OutputPath")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| req.request.get("outputPath").and_then(|v| v.as_str()).map(|s| s.to_string()));
                let config_path = req
                    .request
                    .get("ConfigPath")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| req.request.get("configPath").and_then(|v| v.as_str()).map(|s| s.to_string()));
                EffectiveRequest {
                    docker_only,
                    no_docker,
                    output_json,
                    output_path,
                    config_path,
                }
            }
            Ok(None) => {
                let args: Vec<String> = std::env::args().collect();
                let args = normalize_legacy_args(args);
                let cli = Cli::parse_from(args);
                EffectiveRequest {
                    docker_only: cli.docker_only,
                    no_docker: cli.no_docker,
                    output_json: cli.output_json,
                    output_path: cli.output_path,
                    config_path: cli.config_path,
                }
            }
            Err(e) => return Err(anyhow!(e)),
        };

        effective_request = Some(effective.clone());
        let (cfg, used_config_path) = load_config(effective.config_path.as_deref())?;
        feedback.push(FeedbackEntry::info("init", &format!("Config: {}", used_config_path)));

        let mut docker_result = json!({});
        if !effective.no_docker {
            feedback.push(FeedbackEntry::info("docker", "Levantando servicios Docker"));
            let compose_path = if cfg.docker_compose_path.is_empty() {
                "docker-compose.yml".to_string()
            } else {
                cfg.docker_compose_path.clone()
            };
            let services = if cfg.docker_services.is_empty() {
                vec!["gesfer-db".to_string(), "cache".to_string(), "adminer".to_string()]
            } else {
                cfg.docker_services.clone()
            };
            docker_compose_up(&compose_path, &services)?;
            docker_result = json!({
                "composePath": compose_path,
                "services": services
            });

            feedback.push(FeedbackEntry::info("mysql", "Esperando MySQL listo"));
            let mysql_container = if cfg.mysql_container_name.is_empty() {
                "gesfer_db".to_string()
            } else {
                cfg.mysql_container_name.clone()
            };
            wait_mysql_ready(&mysql_container, &cfg.health_check)?;
        } else {
            feedback.push(FeedbackEntry::info("docker", "Saltando Docker (--no-docker)"));
        }

        let mut clients_started = vec![];
        if !effective.docker_only {
            if !cfg.start_clients.is_empty() {
                feedback.push(FeedbackEntry::info("clients", "Iniciando clientes (best-effort)"));
                clients_started = start_clients(&cfg.start_clients)?;
            } else {
                feedback.push(FeedbackEntry::info("clients", "Sin clientes configurados"));
            }
        } else {
            feedback.push(FeedbackEntry::info("clients", "Saltando clientes (--docker-only)"));
        }

        Ok(json!({
            "configPath": used_config_path,
            "docker": docker_result,
            "clients": clients_started
        }))
    })();

    let (success, exit_code, message, result) = match run {
        Ok(res) => {
            feedback.push(FeedbackEntry::info("done", "Entorno preparado"));
            (true, 0, "OK", res)
        }
        Err(e) => {
            feedback.push(FeedbackEntry::error("error", "Fallo en prepare-full-env", Some(&e.to_string())));
            (false, 1, "ERROR", json!({ "error": e.to_string() }))
        }
    };

    let duration_ms = Some(started_at.elapsed().as_millis() as u64);
    let res = CapsuleResponse::tool(tool_id, success, exit_code, message, feedback, result, duration_ms);
    // OutputPath: si está presente, persistimos el JSON completo del envelope.
    if let Some(req) = &effective_request {
        if let Some(p) = &req.output_path {
            if let Ok(s) = serde_json::to_string(&res) {
                let _ = fs::write(p, s);
            }
        }
    }

    // Stdout: en modo cápsula (agente) es obligatorio; en CLI se controla por OutputJson.
    let should_stdout = force_stdout
        || effective_request
            .as_ref()
            .map(|r| r.output_json || r.output_path.is_none())
            .unwrap_or(true);
    if should_stdout {
        let _ = write_capsule_response(&res);
    }
    std::process::exit(res.exit_code);
}

