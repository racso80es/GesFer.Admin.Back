use anyhow::{anyhow, Context, Result};
use clap::Parser;
use gesfer_capsule::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

const DEBUG_LOG_FILE: &str = "debug-b3c63d.log";
const DEBUG_SESSION_ID: &str = "b3c63d";

fn resolve_debug_log_path() -> Option<PathBuf> {
    // 1) Repo root explícito si existe
    if let Ok(root) = std::env::var("GESFER_REPO_ROOT") {
        let p = PathBuf::from(root).join(DEBUG_LOG_FILE);
        return Some(p);
    }
    // 2) CWD actual
    if let Ok(cwd) = std::env::current_dir() {
        return Some(cwd.join(DEBUG_LOG_FILE));
    }
    // 3) Junto al exe (último recurso)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return Some(dir.join(DEBUG_LOG_FILE));
        }
    }
    None
}

fn dbg_log(run_id: &str, hypothesis_id: &str, location: &str, message: &str, data: serde_json::Value) {
    // NDJSON line append. No secretos.
    let payload = json!({
        "sessionId": DEBUG_SESSION_ID,
        "runId": run_id,
        "hypothesisId": hypothesis_id,
        "location": location,
        "message": message,
        "data": data,
        "timestamp": chrono::Utc::now().timestamp_millis()
    });
    if let Ok(line) = serde_json::to_string(&payload) {
        let Some(path) = resolve_debug_log_path() else { return; };
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
            use std::io::Write;
            let _ = writeln!(f, "{}", line);
        } else {
            // best-effort: no panic, no secretos; no stdout/stderr para no romper contrato.
        }
    }
}

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

fn docker_port_3306(container: &str) -> Result<u16> {
    // docker port <container> 3306/tcp  => "0.0.0.0:3306" o "[::]:3306"
    let mut cmd = Command::new("docker");
    cmd.arg("port").arg(container).arg("3306/tcp");
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("docker port falló: {}", text));
    }
    // tomar primer match :<port>
    let first = text.lines().next().unwrap_or("").trim();
    let port_str = first
        .rsplit(':')
        .next()
        .ok_or_else(|| anyhow!("docker port salida inesperada: {}", text))?;
    let port: u16 = port_str
        .trim()
        .parse()
        .with_context(|| format!("No se pudo parsear puerto desde: {}", first))?;
    Ok(port)
}

fn docker_count_tables(container: &str, db: &str, root_pwd: &str) -> Result<u64> {
    // Cuenta tablas reales (excluye system schemas)
    let sql = format!("SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = '{db}';");
    let mut cmd = Command::new("docker");
    cmd.arg("exec")
        .arg(container)
        .arg("mysql")
        .arg("-uroot")
        .arg(format!("-p{}", root_pwd))
        .arg("-N")
        .arg("-s")
        .arg("-e")
        .arg(sql);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("No se pudo contar tablas: {}", text));
    }
    // Puede venir acompañado de warnings (stderr) concatenados por run_cmd; tomamos el primer token numérico.
    let first_token = text
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow!("Salida vacía COUNT(*): {}", text))?;
    let n: u64 = first_token
        .trim()
        .parse()
        .with_context(|| format!("Salida inválida COUNT(*): {}", text))?;
    Ok(n)
}

fn docker_count_rows(container: &str, db: &str, root_pwd: &str, table: &str) -> Result<u64> {
    let sql = format!("USE `{db}`; SELECT COUNT(1) FROM `{table}`;");
    let mut cmd = Command::new("docker");
    cmd.arg("exec")
        .arg(container)
        .arg("mysql")
        .arg("-uroot")
        .arg(format!("-p{}", root_pwd))
        .arg("-N")
        .arg("-s")
        .arg("-e")
        .arg(sql);
    let (code, text) = run_cmd(cmd)?;
    if code != 0 {
        return Err(anyhow!("No se pudo contar filas en {}.{}: {}", db, table, text));
    }
    let first_token = text
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow!("Salida vacía COUNT(1) {}.{}: {}", db, table, text))?;
    let n: u64 = first_token
        .trim()
        .parse()
        .with_context(|| format!("Salida inválida COUNT(1) {}.{}: {}", db, table, text))?;
    Ok(n)
}

fn build_connection_string(host: &str, port: u16, db: &str, user: &str, pwd: &str) -> String {
    // No loguear pwd. Mantener compatibilidad con MySQL 8 / Pomelo.
    format!(
        "Server={host};Port={port};Database={db};User={user};Password={pwd};CharSet=utf8mb4;AllowUserVariables=True;AllowLoadLocalInfile=True;"
    )
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
    let run_id = format!("run-{}", chrono::Utc::now().timestamp_millis());
    let debug_log_path = resolve_debug_log_path().map(|p| p.to_string_lossy().to_string());

    let run = (|| -> Result<serde_json::Value> {
        // Touch debug log (best-effort) so podamos localizarlo incluso si no se escriben eventos.
        if let Some(p) = resolve_debug_log_path() {
            let _ = fs::OpenOptions::new().create(true).append(true).open(p);
        }
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
        dbg_log(
            &run_id,
            "H1",
            "invoke_mysql_seeds.rs:effective_request",
            "EffectiveRequest construido",
            json!({
                "dropCreateDb": effective.drop_create_db,
                "skipMigrations": effective.skip_migrations,
                "skipSeeds": effective.skip_seeds,
                "hasConfigPathArg": effective.config_path.is_some(),
                "hasOutputPath": effective.output_path.is_some(),
                "outputJson": effective.output_json
            }),
        );

        let (cfg, used_config_path) = load_config(effective.config_path.as_deref())?;
        feedback.push(FeedbackEntry::info("init", &format!("Config: {}", used_config_path)));
        dbg_log(
            &run_id,
            "H1",
            "invoke_mysql_seeds.rs:load_config",
            "Config cargada",
            json!({
                "usedConfigPath": used_config_path,
                "runMigrations": cfg.run_migrations,
                "runSeeds": cfg.run_seeds,
                "efProjectEmpty": cfg.ef_project.is_empty(),
                "startupProjectEmpty": cfg.startup_project.is_empty(),
                "seedsProjectEmpty": cfg.seeds_project.is_empty(),
                "connectionEnv": cfg.connection_env
            }),
        );

        let container = if cfg.mysql_container_name.is_empty() {
            "gesfer_db".to_string()
        } else {
            cfg.mysql_container_name.clone()
        };

        feedback.push(FeedbackEntry::info("mysql", &format!("Esperando MySQL listo (container={})", container)));
        wait_mysql_ready(&container, &cfg.health_check)?;

        let db_name = docker_exec_env(&container, "MYSQL_DATABASE")?;
        dbg_log(
            &run_id,
            "H2",
            "invoke_mysql_seeds.rs:mysql_env",
            "Variables MySQL detectadas (no secret)",
            json!({
                "container": container,
                "mysqlDatabase": db_name
            }),
        );

        let mut db_result = json!({
            "name": db_name,
            "dropCreate": { "attempted": false, "dropped": false, "created": false }
        });

        if effective.drop_create_db {
            feedback.push(FeedbackEntry::info("db_drop_create", "Aplicando DROP/CREATE database (estrategia B)"));
            let root_pwd = docker_exec_env(&container, "MYSQL_ROOT_PASSWORD")?;
            dbg_log(
                &run_id,
                "H2",
                "invoke_mysql_seeds.rs:db_drop_create",
                "Ejecutando DROP/CREATE DB (password NO logueada)",
                json!({
                    "container": container,
                    "db": db_name
                }),
            );
            drop_create_database(&container, &db_name, &root_pwd)?;
            db_result["dropCreate"]["attempted"] = json!(true);
            db_result["dropCreate"]["dropped"] = json!(true);
            db_result["dropCreate"]["created"] = json!(true);
        } else {
            feedback.push(FeedbackEntry::info("db_drop_create", "Saltando DROP/CREATE DB (no habilitado)"));
        }

        // Derivar connection string hacia el contenedor usando el puerto host real (evita mismatch con MySQL local u otros puertos).
        // Fuente SSOT: env del contenedor (MYSQL_*).
        let mysql_user = docker_exec_env(&container, "MYSQL_USER")?;
        let mysql_pwd = docker_exec_env(&container, "MYSQL_PASSWORD")?;
        let host_port = docker_port_3306(&container)?;
        let forced_conn = build_connection_string("localhost", host_port, &db_name, &mysql_user, &mysql_pwd);
        dbg_log(
            &run_id,
            "H2",
            "invoke_mysql_seeds.rs:forced_connection",
            "ConnectionStrings__DefaultConnection forzado para dotnet (sin password)",
            json!({
                "host": "localhost",
                "port": host_port,
                "db": db_name,
                "user": mysql_user
            }),
        );

        let mut migrations_result = json!({
            "attempted": false,
            "success": false,
            "efProject": cfg.ef_project,
            "startupProject": cfg.startup_project
        });

        if !effective.skip_migrations && cfg.run_migrations {
            feedback.push(FeedbackEntry::info("migrations", "Ejecutando migraciones EF (dotnet ef database update)"));
            let conn_env = cfg.connection_env.clone();
            let conn_env_is_set = std::env::var(&conn_env).is_ok();
            dbg_log(
                &run_id,
                "H2",
                "invoke_mysql_seeds.rs:before_dotnet_ef",
                "Antes de dotnet ef (sin log de connection string)",
                json!({
                    "connectionEnv": conn_env,
                    "connectionEnvPresent": conn_env_is_set,
                    "efProject": migrations_result["efProject"],
                    "startupProject": migrations_result["startupProject"]
                }),
            );
            // Forzar connection string al destino esperado (contenedor) solo para el proceso dotnet.
            let mut cmd = Command::new("dotnet");
            cmd.arg("ef")
                .arg("database")
                .arg("update")
                .arg("--project")
                .arg(migrations_result["efProject"].as_str().unwrap_or_default())
                .arg("--startup-project")
                .arg(migrations_result["startupProject"].as_str().unwrap_or_default());
            cmd.env(&cfg.connection_env, &forced_conn);
            let (code, text) = run_cmd(cmd)?;
            if code != 0 {
                return Err(anyhow!("dotnet ef database update falló: {}", text));
            }
            migrations_result["attempted"] = json!(true);
            migrations_result["success"] = json!(true);
            dbg_log(
                &run_id,
                "H3",
                "invoke_mysql_seeds.rs:after_dotnet_ef",
                "dotnet ef completó",
                json!({ "success": true }),
            );

            // Evidencia: contar tablas en el contenedor tras migraciones.
            let root_pwd = docker_exec_env(&container, "MYSQL_ROOT_PASSWORD")?;
            let tables = docker_count_tables(&container, &db_name, &root_pwd)?;
            dbg_log(
                &run_id,
                "H3",
                "invoke_mysql_seeds.rs:after_migrations_table_count",
                "Conteo de tablas en contenedor tras migraciones",
                json!({ "tables": tables }),
            );
            // Si no hay tablas, la migración no impactó en el destino esperado => error.
            if tables == 0 {
                return Err(anyhow!("Migraciones reportaron éxito pero la BD en contenedor sigue sin tablas (tables=0). Posible mismatch de conexión."));
            }
        } else {
            feedback.push(FeedbackEntry::info("migrations", "Saltando migraciones (--skip-migrations o config.runMigrations=false)"));
            dbg_log(
                &run_id,
                "H1",
                "invoke_mysql_seeds.rs:skip_migrations",
                "Migraciones saltadas",
                json!({
                    "skipMigrationsFlag": effective.skip_migrations,
                    "configRunMigrations": cfg.run_migrations
                }),
            );
        }

        let mut seeds_result = json!({
            "attempted": false,
            "success": false,
            "mode": "RUN_SEEDS_ONLY=1",
            "seedsProject": cfg.seeds_project
        });

        if !effective.skip_seeds && cfg.run_seeds {
            feedback.push(FeedbackEntry::info("seeds", "Ejecutando seeds (API con RUN_SEEDS_ONLY=1)"));
            dbg_log(
                &run_id,
                "H4",
                "invoke_mysql_seeds.rs:before_seeds",
                "Antes de dotnet run seeds",
                json!({
                    "seedsProject": seeds_result["seedsProject"],
                    "connectionEnv": cfg.connection_env,
                    "connectionEnvPresent": std::env::var(&cfg.connection_env).is_ok()
                }),
            );
            // Forzar connection string al destino esperado (contenedor) solo para el proceso dotnet.
            let mut cmd = Command::new("dotnet");
            cmd.arg("run")
                .arg("--project")
                .arg(seeds_result["seedsProject"].as_str().unwrap_or_default());
            cmd.env("RUN_SEEDS_ONLY", "1");
            cmd.env(&cfg.connection_env, &forced_conn);
            let (code, text) = run_cmd(cmd)?;
            if code != 0 {
                return Err(anyhow!("dotnet run (RUN_SEEDS_ONLY=1) falló: {}", text));
            }
            seeds_result["attempted"] = json!(true);
            seeds_result["success"] = json!(true);
            dbg_log(
                &run_id,
                "H4",
                "invoke_mysql_seeds.rs:after_seeds",
                "Seeds completados",
                json!({ "success": true }),
            );

            // Evidencia: contar tablas tras seeds (debe seguir >0).
            let root_pwd = docker_exec_env(&container, "MYSQL_ROOT_PASSWORD")?;
            let tables = docker_count_tables(&container, &db_name, &root_pwd)?;
            dbg_log(
                &run_id,
                "H4",
                "invoke_mysql_seeds.rs:after_seeds_table_count",
                "Conteo de tablas en contenedor tras seeds",
                json!({ "tables": tables }),
            );

            // Evidencia adicional: conteo de filas en tablas seed clave (best-effort).
            // Nota: nombres de tabla según migraciones actuales (singular: Language/Country/State/City/PostalCode).
            let candidates = ["Language", "Country", "State", "City", "PostalCode", "Companies", "AdminUsers"];
            let mut row_counts = serde_json::Map::new();
            let mut any_rows = false;
            for t in candidates {
                match docker_count_rows(&container, &db_name, &root_pwd, t) {
                    Ok(n) => {
                        if n > 0 {
                            any_rows = true;
                        }
                        row_counts.insert(t.to_string(), json!(n));
                    }
                    Err(e) => {
                        // No fallamos por tablas ausentes; dejamos trazabilidad.
                        row_counts.insert(t.to_string(), json!({ "error": e.to_string() }));
                    }
                }
            }
            seeds_result["rowCounts"] = json!(row_counts);
            dbg_log(
                &run_id,
                "H4",
                "invoke_mysql_seeds.rs:after_seeds_row_counts",
                "Conteo filas seed (best-effort)",
                json!({ "rowCounts": seeds_result["rowCounts"], "anyRows": any_rows }),
            );

            if !any_rows {
                return Err(anyhow!("Seeds reportaron éxito pero no se observa ninguna fila en tablas seed clave (Languages/Countries/Companies/AdminUsers)."));
            }
        } else {
            feedback.push(FeedbackEntry::info("seeds", "Saltando seeds (--skip-seeds o config.runSeeds=false)"));
            dbg_log(
                &run_id,
                "H1",
                "invoke_mysql_seeds.rs:skip_seeds",
                "Seeds saltados",
                json!({
                    "skipSeedsFlag": effective.skip_seeds,
                    "configRunSeeds": cfg.run_seeds
                }),
            );
        }

        Ok(json!({
            "configPath": used_config_path,
            "debugLogPath": debug_log_path,
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

