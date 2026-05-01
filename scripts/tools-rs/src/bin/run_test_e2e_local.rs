use std::{fs, time::Duration};

use clap::Parser;
use gesfer_tools::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

const TOOL_ID: &str = "run-test-e2e-local";

// Exit codes por rango:
// - 10–13 smoke
// - 20–21 company read
// - 30–34 company CRUD
// - 40–44 user CRUD
const EXIT_HEALTH: i32 = 10;
const EXIT_SWAGGER: i32 = 11;
const EXIT_LOGIN_OK: i32 = 12;
const EXIT_LOGIN_INVALID_UNAUTHORIZED: i32 = 13;

const EXIT_COMPANY_LIST_JWT: i32 = 20;
const EXIT_COMPANY_READ_SECRET: i32 = 21;

const EXIT_COMPANY_CREATE: i32 = 30;
const EXIT_COMPANY_UPDATE: i32 = 31;
const EXIT_COMPANY_DELETE: i32 = 32;
const EXIT_COMPANY_VERIFY_404: i32 = 33;
const EXIT_COMPANY_UNAUTHORIZED: i32 = 34;

const EXIT_USER_CREATE: i32 = 40;
const EXIT_USER_READ: i32 = 41;
const EXIT_USER_UPDATE: i32 = 42;
const EXIT_USER_DELETE: i32 = 43;
const EXIT_USER_VERIFY_404: i32 = 44;

#[derive(Debug, Parser)]
#[command(name = "run_test_e2e_local", version, about = "Pruebas E2E HTTP contra API Admin local (tools contract v2)")]
struct Cli {
    #[arg(long = "base-url")]
    base_url: Option<String>,
    #[arg(long = "config-path")]
    config_path: Option<String>,

    #[arg(long = "run-smoke")]
    run_smoke: Option<bool>,
    #[arg(long = "run-company-read")]
    run_company_read: Option<bool>,
    #[arg(long = "run-company-crud")]
    run_company_crud: Option<bool>,
    #[arg(long = "run-user-crud")]
    run_user_crud: Option<bool>,

    #[arg(long = "demo-company-id")]
    demo_company_id: Option<String>,
    #[arg(long = "demo-company-name")]
    demo_company_name: Option<String>,

    #[arg(long)]
    admin_user: Option<String>,
    #[arg(long)]
    admin_password: Option<String>,
    #[arg(long)]
    internal_secret: Option<String>,

    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    output_json: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct JsonRequest {
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    config_path: Option<String>,

    #[serde(default)]
    run_smoke: Option<bool>,
    #[serde(default)]
    run_company_read: Option<bool>,
    #[serde(default)]
    run_company_crud: Option<bool>,
    #[serde(default)]
    run_user_crud: Option<bool>,

    #[serde(default)]
    demo_company_id: Option<String>,
    #[serde(default)]
    demo_company_name: Option<String>,

    #[serde(default)]
    admin_user: Option<String>,
    #[serde(default)]
    admin_password: Option<String>,
    #[serde(default)]
    internal_secret: Option<String>,

    #[serde(default)]
    output_path: Option<String>,
    #[serde(default)]
    output_json: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ToolConfig {
    #[serde(default)]
    default_base_url: Option<String>,
    #[serde(default)]
    demo_company_id: Option<String>,
    #[serde(default)]
    demo_company_name: Option<String>,
    #[serde(default)]
    http_timeout_seconds: Option<u64>,
}

fn main() {
    let started = std::time::Instant::now();
    let mut feedback = vec![FeedbackEntry::info("init", "Iniciando run-test-e2e-local (Rust)")];

    let capsule_req = try_read_capsule_request().ok().flatten();
    let mut cli = Cli::parse();

    // Modo agente: el envelope puede sobreescribir CLI.
    if let Some(req) = capsule_req {
        let jreq = serde_json::from_value::<JsonRequest>(req.request).unwrap_or_default();
        if jreq.base_url.is_some() {
            cli.base_url = jreq.base_url.clone();
        }
        if jreq.config_path.is_some() {
            cli.config_path = jreq.config_path.clone();
        }
        if jreq.run_smoke.is_some() {
            cli.run_smoke = jreq.run_smoke;
        }
        if jreq.run_company_read.is_some() {
            cli.run_company_read = jreq.run_company_read;
        }
        if jreq.run_company_crud.is_some() {
            cli.run_company_crud = jreq.run_company_crud;
        }
        if jreq.run_user_crud.is_some() {
            cli.run_user_crud = jreq.run_user_crud;
        }
        if jreq.demo_company_id.is_some() {
            cli.demo_company_id = jreq.demo_company_id.clone();
        }
        if jreq.demo_company_name.is_some() {
            cli.demo_company_name = jreq.demo_company_name.clone();
        }
        if jreq.admin_user.is_some() {
            cli.admin_user = jreq.admin_user.clone();
        }
        if jreq.admin_password.is_some() {
            cli.admin_password = jreq.admin_password.clone();
        }
        if jreq.internal_secret.is_some() {
            cli.internal_secret = jreq.internal_secret.clone();
        }
        if jreq.output_path.is_some() {
            cli.output_path = jreq.output_path.clone();
        }
        if let Some(v) = jreq.output_json {
            cli.output_json = v;
        }
    }

    let cfg = load_config(cli.config_path.as_deref(), &mut feedback);
    let base_url = normalize_base_url(
        cli.base_url
            .clone()
            .or_else(|| std::env::var("E2E_BASE_URL").ok())
            .or_else(|| cfg.default_base_url.clone())
            .unwrap_or_else(|| "http://localhost:5010".to_string()),
    );

    let timeout = Duration::from_secs(cfg.http_timeout_seconds.unwrap_or(60));
    let client = Client::builder().timeout(timeout).build();
    let client = match client {
        Ok(c) => c,
        Err(e) => {
            feedback.push(FeedbackEntry::error("init", "No se pudo construir cliente HTTP", Some(&e.to_string())));
            let res = CapsuleResponse::tool(
                TOOL_ID,
                false,
                3,
                "Init HTTP falló",
                feedback,
                serde_json::json!({ "baseUrl": base_url }),
                Some(started.elapsed().as_millis() as u64),
            );
            emit_and_exit(res, cli.output_path, cli.output_json);
        }
    };

    let run_smoke = cli.run_smoke.unwrap_or(true);
    let run_company_read = cli.run_company_read.unwrap_or(true);
    let run_company_crud = cli.run_company_crud.unwrap_or(true);
    let run_user_crud = cli.run_user_crud.unwrap_or(true);

    let demo_company_id = cli
        .demo_company_id
        .clone()
        .or_else(|| cfg.demo_company_id.clone())
        .unwrap_or_else(|| "11111111-1111-1111-1111-111111111115".to_string());
    let demo_company_name = cli
        .demo_company_name
        .clone()
        .or_else(|| cfg.demo_company_name.clone())
        .unwrap_or_else(|| "Empresa Demo".to_string());

    let admin_user = cli
        .admin_user
        .clone()
        .or_else(|| std::env::var("E2E_ADMIN_USER").ok())
        .unwrap_or_else(|| "admin".to_string());
    let admin_password = cli
        .admin_password
        .clone()
        .or_else(|| std::env::var("E2E_ADMIN_PASSWORD").ok())
        .unwrap_or_else(|| "admin123".to_string());
    let internal_secret = cli
        .internal_secret
        .clone()
        .or_else(|| std::env::var("E2E_INTERNAL_SECRET").ok())
        .unwrap_or_else(|| "dev-internal-secret-change-in-production".to_string());

    let mut result = serde_json::json!({
        "baseUrl": base_url,
        "runSmoke": run_smoke,
        "runCompanyRead": run_company_read,
        "runCompanyCrud": run_company_crud,
        "runUserCrud": run_user_crud,
        "smoke": { "healthOk": null, "swaggerOk": null, "loginOk": null, "loginInvalidUnauthorized": null, "steps": [] },
        "companyRead": { "listJwtOk": null, "empresaDemoInList": null, "listSecretOk": null, "getByIdOk": null, "unauthorizedWithoutAuth": null, "steps": [] },
        "companyCrud": { "ok": null, "companyId": null, "steps": [] },
        "userCrud": { "ok": null, "userId": null, "steps": [] }
    });

    // Pre: JWT admin (solo si alguna fase lo necesita).
    let mut admin_jwt: Option<String> = None;
    if run_company_read {
        match admin_login(&client, &base_url, &admin_user, &admin_password, &mut feedback) {
            Ok(t) => admin_jwt = Some(t),
            Err(e) => {
                set_smoke_step_err(&mut result, "loginOk", &e);
                feedback.push(FeedbackEntry::error("companyRead", "login admin falló", Some(&e)));
                let res = CapsuleResponse::tool(
                    TOOL_ID,
                    false,
                    EXIT_LOGIN_OK,
                    "Fallo en login admin (pre company read)",
                    feedback,
                    result,
                    Some(started.elapsed().as_millis() as u64),
                );
                emit_and_exit(res, cli.output_path, cli.output_json);
            }
        }
    }

    // 1) Smoke
    if run_smoke {
        if let Err(e) = run_smoke_phase(&client, &base_url, &admin_user, &admin_password, &mut feedback, &mut result)
        {
            feedback.push(FeedbackEntry::error("smoke", "fase fallida", Some(&e.detail)));
            let res = CapsuleResponse::tool(
                TOOL_ID,
                false,
                e.exit_code,
                "Smoke falló",
                feedback,
                result,
                Some(started.elapsed().as_millis() as u64),
            );
            emit_and_exit(res, cli.output_path, cli.output_json);
        }
    } else {
        mark_skipped(&mut result, "smoke");
    }

    // 2) Company read
    if run_company_read {
        let jwt = admin_jwt.clone().unwrap_or_default();
        if let Err(e) = run_company_read_phase(
            &client,
            &base_url,
            &jwt,
            &internal_secret,
            &demo_company_id,
            &demo_company_name,
            &mut feedback,
            &mut result,
        ) {
            feedback.push(FeedbackEntry::error("companyRead", "fase fallida", Some(&e.detail)));
            let res = CapsuleResponse::tool(
                TOOL_ID,
                false,
                e.exit_code,
                "Company read falló",
                feedback,
                result,
                Some(started.elapsed().as_millis() as u64),
            );
            emit_and_exit(res, cli.output_path, cli.output_json);
        }
    } else {
        mark_skipped(&mut result, "companyRead");
    }

    // 3) Company CRUD (solo internal secret)
    if run_company_crud {
        if let Err(e) = run_company_crud_phase(
            &client,
            &base_url,
            &internal_secret,
            &mut feedback,
            &mut result,
        ) {
            feedback.push(FeedbackEntry::error("companyCrud", "fase fallida", Some(&e.detail)));
            let res = CapsuleResponse::tool(
                TOOL_ID,
                false,
                e.exit_code,
                "Company CRUD falló",
                feedback,
                result,
                Some(started.elapsed().as_millis() as u64),
            );
            emit_and_exit(res, cli.output_path, cli.output_json);
        }
    } else {
        mark_skipped(&mut result, "companyCrud");
    }

    // 4) User CRUD (solo internal secret; secuencial + autolimpieza)
    if run_user_crud {
        if let Err(e) = run_user_crud_phase(
            &client,
            &base_url,
            &internal_secret,
            &demo_company_id,
            &mut feedback,
            &mut result,
        ) {
            feedback.push(FeedbackEntry::error("userCrud", "fase fallida", Some(&e.detail)));
            let res = CapsuleResponse::tool(
                TOOL_ID,
                false,
                e.exit_code,
                "User CRUD falló",
                feedback,
                result,
                Some(started.elapsed().as_millis() as u64),
            );
            emit_and_exit(res, cli.output_path, cli.output_json);
        }
    } else {
        mark_skipped(&mut result, "userCrud");
    }

    let res = CapsuleResponse::tool(
        TOOL_ID,
        true,
        0,
        "E2E OK",
        feedback,
        result,
        Some(started.elapsed().as_millis() as u64),
    );
    emit_and_exit(res, cli.output_path, cli.output_json);
}

#[derive(Debug)]
struct PhaseError {
    exit_code: i32,
    detail: String,
}

fn emit_and_exit(res: CapsuleResponse, output_path: Option<String>, output_json: bool) -> ! {
    if let Some(p) = output_path {
        let _ = fs::write(p, serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string()));
    }
    if output_json {
        let _ = write_capsule_response(&res);
    }
    std::process::exit(res.exit_code);
}

fn normalize_base_url(base_url: String) -> String {
    base_url.trim().trim_end_matches('/').to_string()
}

fn load_config(config_path: Option<&str>, feedback: &mut Vec<FeedbackEntry>) -> ToolConfig {
    let path = config_path.map(|s| s.to_string()).unwrap_or_else(|| {
        // Preferir ruta dentro de la cápsula cuando se ejecuta desde raíz del repo.
        // Fallback: fichero al lado del .exe si el cwd ya es la cápsula.
        "scripts/tools/run-test-e2e-local/run-test-e2e-local-config.json".to_string()
    });
    let candidates = vec![
        path.clone(),
        "run-test-e2e-local-config.json".to_string(),
        "run-test-e2e-local-config.json".to_string(),
    ];
    let mut last_err: Option<String> = None;
    for p in candidates {
        match fs::read_to_string(&p) {
            Ok(s) => match serde_json::from_str::<ToolConfig>(&s) {
                Ok(cfg) => {
                    feedback.push(FeedbackEntry::info("config", &format!("Config cargada: {p}")));
                    return cfg;
                }
                Err(e) => {
                    feedback.push(FeedbackEntry::warning("config", "Config inválida; usando defaults", Some(&e.to_string())));
                    return ToolConfig::default();
                }
            },
            Err(e) => last_err = Some(format!("{p}: {e}")),
        }
    }
    feedback.push(FeedbackEntry::warning(
        "config",
        "Config no encontrada; usando defaults",
        last_err.as_deref(),
    ));
    ToolConfig::default()
}

fn mark_skipped(result: &mut serde_json::Value, key: &str) {
    if let Some(obj) = result.get_mut(key) {
        if let Some(steps) = obj.get_mut("steps").and_then(|v| v.as_array_mut()) {
            steps.push(serde_json::json!({ "step": "skipped", "ok": true }));
        }
        if let Some(ok) = obj.get_mut("ok") {
            *ok = serde_json::json!(true);
        }
    }
}

fn set_smoke_step_err(result: &mut serde_json::Value, field: &str, detail: &str) {
    if let Some(smoke) = result.get_mut("smoke") {
        if let Some(v) = smoke.get_mut(field) {
            *v = serde_json::json!(false);
        }
        if let Some(steps) = smoke.get_mut("steps").and_then(|v| v.as_array_mut()) {
            steps.push(serde_json::json!({ "step": field, "ok": false, "detail": detail }));
        }
    }
}

fn admin_login(client: &Client, base_url: &str, user: &str, pass: &str, feedback: &mut Vec<FeedbackEntry>) -> Result<String, String> {
    feedback.push(FeedbackEntry::info("auth", "POST /api/admin/auth/login"));
    let url = format!("{base_url}/api/admin/auth/login");
    let resp = client
        .post(&url)
        .header("Accept", "application/json")
        .json(&serde_json::json!({ "Usuario": user, "Contraseña": pass }))
        .send()
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("login status {}", resp.status()));
    }
    let body: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    let token = body.get("token").and_then(|v| v.as_str()).unwrap_or("").to_string();
    if token.trim().is_empty() {
        return Err("token vacío".to_string());
    }
    Ok(token)
}

fn run_smoke_phase(
    client: &Client,
    base_url: &str,
    admin_user: &str,
    admin_password: &str,
    feedback: &mut Vec<FeedbackEntry>,
    result: &mut serde_json::Value,
) -> Result<(), PhaseError> {
    // health
    feedback.push(FeedbackEntry::info("smoke", "GET /health"));
    let health = client.get(format!("{base_url}/health")).send();
    match health {
        Ok(r) if r.status() == StatusCode::OK => {
            result["smoke"]["healthOk"] = serde_json::json!(true);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "health", "ok": true }));
        }
        Ok(r) => {
            result["smoke"]["healthOk"] = serde_json::json!(false);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "health", "ok": false, "detail": format!("status {}", r.status()) }));
            return Err(PhaseError { exit_code: EXIT_HEALTH, detail: format!("health status {}", r.status()) });
        }
        Err(e) => {
            result["smoke"]["healthOk"] = serde_json::json!(false);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "health", "ok": false, "detail": e.to_string() }));
            return Err(PhaseError { exit_code: EXIT_HEALTH, detail: e.to_string() });
        }
    }

    // swagger json
    feedback.push(FeedbackEntry::info("smoke", "GET /swagger/v1/swagger.json"));
    let swagger = client.get(format!("{base_url}/swagger/v1/swagger.json")).send();
    match swagger {
        Ok(r) if r.status() == StatusCode::OK => {
            result["smoke"]["swaggerOk"] = serde_json::json!(true);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "swagger", "ok": true }));
        }
        Ok(r) => {
            result["smoke"]["swaggerOk"] = serde_json::json!(false);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "swagger", "ok": false, "detail": format!("status {}", r.status()) }));
            return Err(PhaseError { exit_code: EXIT_SWAGGER, detail: format!("swagger status {}", r.status()) });
        }
        Err(e) => {
            result["smoke"]["swaggerOk"] = serde_json::json!(false);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "swagger", "ok": false, "detail": e.to_string() }));
            return Err(PhaseError { exit_code: EXIT_SWAGGER, detail: e.to_string() });
        }
    }

    // login ok
    let token = admin_login(client, base_url, admin_user, admin_password, feedback)
        .map_err(|e| PhaseError { exit_code: EXIT_LOGIN_OK, detail: e })?;
    result["smoke"]["loginOk"] = serde_json::json!(true);
    result["smoke"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "loginOk", "ok": true }));

    // login inválido -> 401
    feedback.push(FeedbackEntry::info("smoke", "POST /api/admin/auth/login (invalid)"));
    let url = format!("{base_url}/api/admin/auth/login");
    let resp = client
        .post(&url)
        .header("Accept", "application/json")
        .json(&serde_json::json!({ "Usuario": admin_user, "Contraseña": "wrong" }))
        .send();
    match resp {
        Ok(r) if r.status() == StatusCode::UNAUTHORIZED => {
            result["smoke"]["loginInvalidUnauthorized"] = serde_json::json!(true);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "loginInvalidUnauthorized", "ok": true }));
        }
        Ok(r) => {
            result["smoke"]["loginInvalidUnauthorized"] = serde_json::json!(false);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "loginInvalidUnauthorized", "ok": false, "detail": format!("status {}", r.status()) }));
            return Err(PhaseError {
                exit_code: EXIT_LOGIN_INVALID_UNAUTHORIZED,
                detail: format!("invalid login status {}", r.status()),
            });
        }
        Err(e) => {
            result["smoke"]["loginInvalidUnauthorized"] = serde_json::json!(false);
            result["smoke"]["steps"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({ "step": "loginInvalidUnauthorized", "ok": false, "detail": e.to_string() }));
            return Err(PhaseError { exit_code: EXIT_LOGIN_INVALID_UNAUTHORIZED, detail: e.to_string() });
        }
    }

    // Evitar warning por variable sin uso en algunas builds futuras
    let _ = token;
    Ok(())
}

fn run_company_read_phase(
    client: &Client,
    base_url: &str,
    jwt: &str,
    internal_secret: &str,
    demo_company_id: &str,
    demo_company_name: &str,
    feedback: &mut Vec<FeedbackEntry>,
    result: &mut serde_json::Value,
) -> Result<(), PhaseError> {
    // List con JWT admin
    feedback.push(FeedbackEntry::info("companyRead", "GET /api/company (JWT)"));
    let r = client
        .get(format!("{base_url}/api/company"))
        .bearer_auth(jwt)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_LIST_JWT, detail: e.to_string() })?;
    if r.status() != StatusCode::OK {
        result["companyRead"]["listJwtOk"] = serde_json::json!(false);
        return Err(PhaseError { exit_code: EXIT_COMPANY_LIST_JWT, detail: format!("status {}", r.status()) });
    }
    let companies: serde_json::Value = r
        .json()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_LIST_JWT, detail: e.to_string() })?;
    result["companyRead"]["listJwtOk"] = serde_json::json!(true);
    result["companyRead"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "listJwt", "ok": true }));

    // Empresa demo en lista (por nombre o id)
    let mut demo_ok = false;
    if let Some(arr) = companies.as_array() {
        demo_ok = arr.iter().any(|c| {
            let id_ok = c
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.eq_ignore_ascii_case(demo_company_id))
                .unwrap_or(false);
            let name_ok = c
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s == demo_company_name)
                .unwrap_or(false);
            id_ok || name_ok
        });
    }
    result["companyRead"]["empresaDemoInList"] = serde_json::json!(demo_ok);

    // List con secret interno
    feedback.push(FeedbackEntry::info("companyRead", "GET /api/company (X-Internal-Secret)"));
    let r2 = client
        .get(format!("{base_url}/api/company"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_READ_SECRET, detail: e.to_string() })?;
    if r2.status() != StatusCode::OK {
        result["companyRead"]["listSecretOk"] = serde_json::json!(false);
        return Err(PhaseError { exit_code: EXIT_COMPANY_READ_SECRET, detail: format!("status {}", r2.status()) });
    }
    result["companyRead"]["listSecretOk"] = serde_json::json!(true);

    // GetById con secret
    feedback.push(FeedbackEntry::info("companyRead", "GET /api/company/{id}"));
    let r3 = client
        .get(format!("{base_url}/api/company/{demo_company_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_READ_SECRET, detail: e.to_string() })?;
    if r3.status() != StatusCode::OK {
        result["companyRead"]["getByIdOk"] = serde_json::json!(false);
        return Err(PhaseError { exit_code: EXIT_COMPANY_READ_SECRET, detail: format!("status {}", r3.status()) });
    }
    result["companyRead"]["getByIdOk"] = serde_json::json!(true);

    // Unauthorized sin auth
    feedback.push(FeedbackEntry::info("companyRead", "GET /api/company (sin auth)"));
    let r4 = client
        .get(format!("{base_url}/api/company"))
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_UNAUTHORIZED, detail: e.to_string() })?;
    let unauthorized_ok = r4.status() == StatusCode::UNAUTHORIZED;
    result["companyRead"]["unauthorizedWithoutAuth"] = serde_json::json!(unauthorized_ok);

    result["companyRead"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "listSecret", "ok": true }));
    result["companyRead"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "getById", "ok": true }));
    result["companyRead"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "unauthorizedWithoutAuth", "ok": unauthorized_ok }));

    if !unauthorized_ok {
        return Err(PhaseError { exit_code: EXIT_COMPANY_UNAUTHORIZED, detail: format!("status {}", r4.status()) });
    }
    Ok(())
}

fn run_company_crud_phase(
    client: &Client,
    base_url: &str,
    internal_secret: &str,
    feedback: &mut Vec<FeedbackEntry>,
    result: &mut serde_json::Value,
) -> Result<(), PhaseError> {
    let unique = Uuid::new_v4().to_string();
    let create = serde_json::json!({
        "Name": format!("Empresa E2E {unique}"),
        "TaxId": "B12345674",
        "Address": "Calle E2E 1",
        "Phone": "911111111",
        "Email": format!("e2e-{unique}@integration.local")
    });

    feedback.push(FeedbackEntry::info("companyCrud", "POST /api/company"));
    let r = client
        .post(format!("{base_url}/api/company"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .json(&create)
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_CREATE, detail: e.to_string() })?;
    if r.status() != StatusCode::CREATED {
        return Err(PhaseError { exit_code: EXIT_COMPANY_CREATE, detail: format!("status {}", r.status()) });
    }
    let created: serde_json::Value = r
        .json()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_CREATE, detail: e.to_string() })?;
    let company_id = created
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if company_id.trim().is_empty() {
        return Err(PhaseError { exit_code: EXIT_COMPANY_CREATE, detail: "id vacío".to_string() });
    }
    result["companyCrud"]["companyId"] = serde_json::json!(company_id);
    result["companyCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "create", "ok": true }));

    let company_id = result["companyCrud"]["companyId"].as_str().unwrap_or("").to_string();

    // Update
    let update = serde_json::json!({
        "Name": format!("Empresa E2E Updated {unique}"),
        "TaxId": "B87654315",
        "Address": "Calle E2E 2",
        "Phone": "922222222",
        "Email": format!("e2e-updated-{unique}@integration.local"),
        "IsActive": true
    });
    feedback.push(FeedbackEntry::info("companyCrud", "PUT /api/company/{id}"));
    let r2 = client
        .put(format!("{base_url}/api/company/{company_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .json(&update)
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_UPDATE, detail: e.to_string() })?;
    if r2.status() != StatusCode::OK {
        return Err(PhaseError { exit_code: EXIT_COMPANY_UPDATE, detail: format!("status {}", r2.status()) });
    }
    let updated: serde_json::Value = r2
        .json()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_UPDATE, detail: e.to_string() })?;
    let updated_name = updated.get("name").and_then(|v| v.as_str()).unwrap_or("");
    if updated_name != update.get("Name").and_then(|v| v.as_str()).unwrap_or("") {
        return Err(PhaseError { exit_code: EXIT_COMPANY_UPDATE, detail: "nombre no actualizado".to_string() });
    }
    result["companyCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "update", "ok": true }));

    // Delete
    feedback.push(FeedbackEntry::info("companyCrud", "DELETE /api/company/{id}"));
    let r3 = client
        .delete(format!("{base_url}/api/company/{company_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_DELETE, detail: e.to_string() })?;
    if r3.status() != StatusCode::NO_CONTENT {
        return Err(PhaseError { exit_code: EXIT_COMPANY_DELETE, detail: format!("status {}", r3.status()) });
    }
    result["companyCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "delete", "ok": true }));

    // Verify 404
    feedback.push(FeedbackEntry::info("companyCrud", "GET /api/company/{id} (expect 404)"));
    let r4 = client
        .get(format!("{base_url}/api/company/{company_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_COMPANY_VERIFY_404, detail: e.to_string() })?;
    if r4.status() != StatusCode::NOT_FOUND {
        return Err(PhaseError { exit_code: EXIT_COMPANY_VERIFY_404, detail: format!("status {}", r4.status()) });
    }
    result["companyCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "verify404", "ok": true }));

    result["companyCrud"]["ok"] = serde_json::json!(true);
    Ok(())
}

fn run_user_crud_phase(
    client: &Client,
    base_url: &str,
    internal_secret: &str,
    demo_company_id: &str,
    feedback: &mut Vec<FeedbackEntry>,
    result: &mut serde_json::Value,
) -> Result<(), PhaseError> {
    let unique = Uuid::new_v4().to_string();
    let company_id = demo_company_id;

    // Step 1: Create
    let create = serde_json::json!({
        "CompanyId": company_id,
        "Username": format!("e2e_user_{unique}"),
        "Password": "TmpPassw0rd!",
        "FirstName": "E2E",
        "LastName": format!("User_{unique}"),
        "Email": format!("e2e-user-{unique}@gesfer.local"),
        "Phone": "900000000",
        "Address": "Calle E2E User 1"
    });

    feedback.push(FeedbackEntry::info("userCrud", "POST /api/user"));
    let r = client
        .post(format!("{base_url}/api/user"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .json(&create)
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_CREATE, detail: e.to_string() })?;
    if r.status() != StatusCode::CREATED {
        return Err(PhaseError { exit_code: EXIT_USER_CREATE, detail: format!("status {}", r.status()) });
    }
    let created: serde_json::Value = r
        .json()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_CREATE, detail: e.to_string() })?;
    let user_id = created
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if user_id.trim().is_empty() {
        return Err(PhaseError { exit_code: EXIT_USER_CREATE, detail: "id vacío".to_string() });
    }
    result["userCrud"]["userId"] = serde_json::json!(user_id);
    result["userCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "create", "ok": true }));

    let user_id = result["userCrud"]["userId"].as_str().unwrap_or("").to_string();

    // Step 2: Read (con companyId en query por llamadas System)
    feedback.push(FeedbackEntry::info("userCrud", "GET /api/user/{id}?companyId=..."));
    let r2 = client
        .get(format!("{base_url}/api/user/{user_id}?companyId={company_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_READ, detail: e.to_string() })?;
    if r2.status() != StatusCode::OK {
        return Err(PhaseError { exit_code: EXIT_USER_READ, detail: format!("status {}", r2.status()) });
    }
    let read: serde_json::Value = r2
        .json()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_READ, detail: e.to_string() })?;
    let read_id = read.get("id").and_then(|v| v.as_str()).unwrap_or("");
    if !read_id.eq_ignore_ascii_case(&user_id) {
        return Err(PhaseError { exit_code: EXIT_USER_READ, detail: "mapeo id incorrecto".to_string() });
    }
    result["userCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "read", "ok": true }));

    // Step 3: Update
    let update = serde_json::json!({
        "Username": format!("e2e_user_{unique}_upd"),
        "FirstName": "E2E",
        "LastName": format!("UserUpdated_{unique}"),
        "Email": format!("e2e-user-upd-{unique}@gesfer.local"),
        "Phone": "900000001",
        "Address": "Calle E2E User 2",
        "IsActive": true
    });
    feedback.push(FeedbackEntry::info("userCrud", "PUT /api/user/{id}"));
    let r3 = client
        .put(format!("{base_url}/api/user/{user_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .json(&update)
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_UPDATE, detail: e.to_string() })?;
    if r3.status() != StatusCode::OK {
        return Err(PhaseError { exit_code: EXIT_USER_UPDATE, detail: format!("status {}", r3.status()) });
    }
    let updated: serde_json::Value = r3
        .json()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_UPDATE, detail: e.to_string() })?;
    let updated_username = updated.get("username").and_then(|v| v.as_str()).unwrap_or("");
    if updated_username != update.get("Username").and_then(|v| v.as_str()).unwrap_or("") {
        return Err(PhaseError { exit_code: EXIT_USER_UPDATE, detail: "username no actualizado".to_string() });
    }
    result["userCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "update", "ok": true }));

    // Step 4: Delete (autolimpieza)
    feedback.push(FeedbackEntry::info("userCrud", "DELETE /api/user/{id}"));
    let r4 = client
        .delete(format!("{base_url}/api/user/{user_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_DELETE, detail: e.to_string() })?;
    if r4.status() != StatusCode::NO_CONTENT {
        return Err(PhaseError { exit_code: EXIT_USER_DELETE, detail: format!("status {}", r4.status()) });
    }
    result["userCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "delete", "ok": true }));

    // Step 5: Verify strict 404
    feedback.push(FeedbackEntry::info("userCrud", "GET /api/user/{id}?companyId=... (expect 404)"));
    let r5 = client
        .get(format!("{base_url}/api/user/{user_id}?companyId={company_id}"))
        .header("X-Internal-Secret", internal_secret)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| PhaseError { exit_code: EXIT_USER_VERIFY_404, detail: e.to_string() })?;
    if r5.status() != StatusCode::NOT_FOUND {
        return Err(PhaseError { exit_code: EXIT_USER_VERIFY_404, detail: format!("status {}", r5.status()) });
    }
    result["userCrud"]["steps"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "step": "verify404", "ok": true }));

    result["userCrud"]["ok"] = serde_json::json!(true);
    Ok(())
}

