use anyhow::{anyhow, Context, Result};
use clap::Parser;
use gesfer_capsule::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MySqlSeedsConfig {
    #[serde(default)]
    ef_project: String,
    #[serde(default)]
    startup_project: String,
    #[serde(default)]
    seeds_project: String,
    #[serde(default)]
    seeds_path: String,
    #[serde(default)]
    connection_env: String,
    #[serde(default)]
    run_migrations: bool,
    #[serde(default)]
    run_seeds: bool,
    #[serde(default)]
    mysql_container_name: String,
    #[serde(default)]
    health_check: HealthCheck,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HealthCheck {
    #[serde(default = "default_mysql_ping_max_attempts")]
    mysql_ping_max_attempts: u32,
    #[serde(default = "default_mysql_ping_retry_seconds")]
    mysql_ping_retry_seconds: u64,
}

fn default_mysql_ping_max_attempts() -> u32 {
    15
}
fn default_mysql_ping_retry_seconds() -> u64 {
    2
}

#[derive(Parser, Debug)]
#[command(name = "invoke_mysql_seeds", disable_help_subcommand = true)]
struct Cli {
    #[arg(long)]
    drop_create_db: bool,
    #[arg(long)]
    skip_migrations: bool,
    #[arg(long)]
    skip_seeds: bool,
    #[arg(long)]
    output_json: bool,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    config_path: Option<String>,
}

#[derive(Debug, Clone)]
struct EffectiveRequest {
    drop_create_db: bool,
    skip_migrations: bool,
    skip_seeds: bool,
    output_json: bool,
    output_path: Option<String>,
    config_path: Option<String>,
}

fn normalize_legacy_args(mut args: Vec<String>) -> Vec<String> {
    // Soportar invocación humana heredada (PowerShell-like) usada por el .bat: -SkipMigrations, -SkipSeeds, -DropCreateDb, ...
    // Se traduce a flags clap: --skip-migrations, --skip-seeds, --drop-create-db, ...
    for a in &mut args {
        let lower = a.to_ascii_lowercase();
        match lower.as_str() {
            "-skipmigrations" => *a = "--skip-migrations".to_string(),
            "-skipseeds" => *a = "--skip-seeds".to_string(),
            "-dropcreatedb" => *a = "--drop-create-db".to_string(),
            "-outputjson" => *a = "--output-json".to_string(),
            "-outputpath" => *a = "--output-path".to_string(),
            "-configpath" => *a = "--config-path".to_string(),
            _ => {}
        }
    }
    args
}

fn resolve_config_path(config_path: Option<&str>) -> Result<String> {
    if let Some(p) = config_path {
        return Ok(p.to_string());
    }

    // 1) Relativo al repo root (cwd): scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json
    let p1 = PathBuf::from("scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json");
    if p1.exists() {
        return Ok(p1.to_string_lossy().to_string());
    }

    // 2) Junto al exe (cuando se invoca desde la cápsula)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p2 = dir.join("mysql-seeds-config.json");
            if p2.exists() {
                return Ok(p2.to_string_lossy().to_string());
            }
        }
    }

    Err(anyhow!(
        "No se encontró config por defecto. Usa --config-path o coloca mysql-seeds-config.json en la cápsula."
    ))
}

fn load_config(config_path: Option<&str>) -> Result<(MySqlSeedsConfig, String)> {
    let path = resolve_config_path(config_path)?;
    let raw = fs::read_to_string(&path).with_context(|| format!("No se pudo leer config: {}", path))?;
    let cfg: MySqlSeedsConfig =
        serde_json::from_str(&raw).with_context(|| format!("JSON inválido en config: {}", path))?;
    Ok((cfg, path))
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

fn docker_health_status(container: &str) -> Result<String> {
    let mut cmd = Command::new("docker");
    cmd.arg("inspect")
        .arg("-f")
        .arg("{{.State.Health.Status}}")
        .arg(container);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
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
    let delay = Duration::from_secs(hc.mysql_ping_retry_seconds);
    for _ in 0..hc.mysql_ping_max_attempts {
        let health = docker_health_status(container)?;
        if health.eq_ignore_ascii_case("healthy") {
            return Ok(());
        }
        if health.eq_ignore_ascii_case("unknown") && docker_is_running(container)? {
            return Ok(());
        }
        std::thread::sleep(delay);
    }
    Err(anyhow!("MySQL no estuvo listo a tiempo (container={})", container))
}

fn docker_exec_env(container: &str, var: &str) -> Result<String> {
    // MySQL image incluye sh; hacemos printenv para evitar depender de un shell externo.
    let mut cmd = Command::new("docker");
    cmd.arg("exec")
        .arg(container)
        .arg("sh")
        .arg("-lc")
        .arg(format!("printenv {}", var));
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("No se pudo leer env {} en contenedor {}: {}", var, container, text));
    }
    let v = text.trim().to_string();
    if v.is_empty() {
        return Err(anyhow!(
            "Env {} está vacía/no definida en contenedor {}",
            var,
            container
        ));
    }
    Ok(v)
}

fn drop_create_database(container: &str, db: &str, root_pwd: &str) -> Result<()> {
    let sql = format!(
        "DROP DATABASE IF EXISTS `{db}`; CREATE DATABASE `{db}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
    );
    let mut cmd = Command::new("docker");
    cmd.arg("exec")
        .arg(container)
        .arg("mysql")
        .arg("-uroot")
        .arg(format!("-p{}", root_pwd))
        .arg("-e")
        .arg(sql);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("DROP/CREATE DB falló: {}", text));
    }
    Ok(())
}

fn dotnet_ef_update(ef_project: &str, startup_project: &str) -> Result<()> {
    let mut cmd = Command::new("dotnet");
    cmd.arg("ef")
        .arg("database")
        .arg("update")
        .arg("--project")
        .arg(ef_project)
        .arg("--startup-project")
        .arg(startup_project);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("dotnet ef database update falló: {}", text));
    }
    Ok(())
}

fn dotnet_run_seeds(seeds_project: &str) -> Result<()> {
    let mut cmd = Command::new("dotnet");
    cmd.arg("run").arg("--project").arg(seeds_project);
    cmd.env("RUN_SEEDS_ONLY", "1");
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("dotnet run (RUN_SEEDS_ONLY=1) falló: {}", text));
    }
    Ok(())
}

fn main() {
    let started_at = Instant::now();
    let tool_id = "invoke-mysql-seeds";
    let mut feedback: Vec<FeedbackEntry> = vec![FeedbackEntry::info("init", "Inicio invoke-mysql-seeds")];

    let mut effective_request: Option<EffectiveRequest> = None;
    let mut force_stdout = false;

    let run = (|| -> Result<serde_json::Value> {
        let effective = match try_read_capsule_request() {
            Ok(Some(req)) => {
                force_stdout = true;
                let drop_create_db = req
                    .request
                    .get("DropCreateDb")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                    || req
                        .request
                        .get("dropCreateDb")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                let skip_migrations = req
                    .request
                    .get("SkipMigrations")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                    || req
                        .request
                        .get("skipMigrations")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                let skip_seeds = req
                    .request
                    .get("SkipSeeds")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                    || req
                        .request
                        .get("skipSeeds")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                let output_json = req
                    .request
                    .get("OutputJson")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                    || req
                        .request
                        .get("outputJson")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
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
                    drop_create_db,
                    skip_migrations,
                    skip_seeds,
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
                    drop_create_db: cli.drop_create_db,
                    skip_migrations: cli.skip_migrations,
                    skip_seeds: cli.skip_seeds,
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

        let container = if cfg.mysql_container_name.is_empty() {
            "gesfer_db".to_string()
        } else {
            cfg.mysql_container_name.clone()
        };

        feedback.push(FeedbackEntry::info("mysql", &format!("Esperando MySQL listo (container={})", container)));
        wait_mysql_ready(&container, &cfg.health_check)?;

        let db_name = docker_exec_env(&container, "MYSQL_DATABASE")?;

        let mut db_result = json!({
            "name": db_name,
            "dropCreate": { "attempted": false, "dropped": false, "created": false }
        });

        if effective.drop_create_db {
            feedback.push(FeedbackEntry::info("db_drop_create", "Aplicando DROP/CREATE database (estrategia B)"));
            let root_pwd = docker_exec_env(&container, "MYSQL_ROOT_PASSWORD")?;
            drop_create_database(&container, &db_name, &root_pwd)?;
            db_result["dropCreate"]["attempted"] = json!(true);
            db_result["dropCreate"]["dropped"] = json!(true);
            db_result["dropCreate"]["created"] = json!(true);
        } else {
            feedback.push(FeedbackEntry::info("db_drop_create", "Saltando DROP/CREATE DB (no habilitado)"));
        }

        let mut migrations_result = json!({
            "attempted": false,
            "success": false,
            "efProject": cfg.ef_project,
            "startupProject": cfg.startup_project
        });

        if !effective.skip_migrations && cfg.run_migrations {
            feedback.push(FeedbackEntry::info("migrations", "Ejecutando migraciones EF (dotnet ef database update)"));
            dotnet_ef_update(&migrations_result["efProject"].as_str().unwrap_or_default(), &migrations_result["startupProject"].as_str().unwrap_or_default())?;
            migrations_result["attempted"] = json!(true);
            migrations_result["success"] = json!(true);
        } else {
            feedback.push(FeedbackEntry::info("migrations", "Saltando migraciones (--skip-migrations o config.runMigrations=false)"));
        }

        let mut seeds_result = json!({
            "attempted": false,
            "success": false,
            "mode": "RUN_SEEDS_ONLY=1",
            "seedsProject": cfg.seeds_project
        });

        if !effective.skip_seeds && cfg.run_seeds {
            feedback.push(FeedbackEntry::info("seeds", "Ejecutando seeds (API con RUN_SEEDS_ONLY=1)"));
            dotnet_run_seeds(&seeds_result["seedsProject"].as_str().unwrap_or_default())?;
            seeds_result["attempted"] = json!(true);
            seeds_result["success"] = json!(true);
        } else {
            feedback.push(FeedbackEntry::info("seeds", "Saltando seeds (--skip-seeds o config.runSeeds=false)"));
        }

        Ok(json!({
            "configPath": used_config_path,
            "mysql": {
                "containerName": container,
                "ready": true
            },
            "db": db_result,
            "migrations": migrations_result,
            "seeds": seeds_result
        }))
    })();

    let (success, exit_code, message, result) = match run {
        Ok(res) => {
            feedback.push(FeedbackEntry::info("done", "OK"));
            (true, 0, "OK", res)
        }
        Err(e) => {
            feedback.push(FeedbackEntry::error(
                "error",
                "Fallo en invoke-mysql-seeds",
                Some(&e.to_string()),
            ));
            (false, 1, "ERROR", json!({ "error": e.to_string() }))
        }
    };

    let duration_ms = Some(started_at.elapsed().as_millis() as u64);
    let res = CapsuleResponse::tool(tool_id, success, exit_code, message, feedback, result, duration_ms);

    if let Some(req) = &effective_request {
        if let Some(p) = &req.output_path {
            if let Ok(s) = serde_json::to_string(&res) {
                let _ = fs::write(p, s);
            }
        }
    }

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

