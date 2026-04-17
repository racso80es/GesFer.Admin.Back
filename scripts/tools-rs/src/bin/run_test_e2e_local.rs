//! Herramienta run-test-e2e-local: pruebas HTTP contra la API Admin en local (salud, auth, company).
//! Contrato: SddIA/norms/capsule-json-io.md, SddIA/tools/tools-contract.md

use std::time::{Duration, Instant};

use clap::Parser;
use gesfer_tools::{to_contract_json, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

const TOOL_ID: &str = "run-test-e2e-local";

#[derive(Parser, Debug)]
#[command(name = "run_test_e2e_local")]
struct Args {
    /// URL base (ej. http://localhost:5010)
    #[arg(long)]
    base_url: Option<String>,
    /// Ruta del JSON de configuración (relativa a la cápsula o repo)
    #[arg(long)]
    config_path: Option<String>,
    /// true/false/1/0 (por defecto true)
    #[arg(long, default_value = "true")]
    run_smoke: String,
    #[arg(long, default_value = "true")]
    run_company_read: String,
    #[arg(long, default_value = "true")]
    run_company_crud: String,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    output_json: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CapsuleRequestFields {
    base_url: Option<String>,
    config_path: Option<String>,
    #[serde(default)]
    run_smoke: Option<bool>,
    #[serde(default)]
    run_company_read: Option<bool>,
    #[serde(default)]
    run_company_crud: Option<bool>,
    admin_user: Option<String>,
    /// Preferir variable de entorno E2E_ADMIN_PASSWORD; no registrar en feedback.
    admin_password: Option<String>,
    internal_secret: Option<String>,
    demo_company_id: Option<String>,
    demo_company_name: Option<String>,
    output_path: Option<String>,
    output_json: Option<bool>,
}

fn parse_bool_str(s: &str) -> bool {
    matches!(
        s.to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn merge_capsule(args: &mut Args, v: &serde_json::Value) {
    let f: CapsuleRequestFields = serde_json::from_value(v.clone()).unwrap_or_default();
    if let Some(x) = f.base_url {
        args.base_url = Some(x);
    }
    if let Some(x) = f.config_path {
        args.config_path = Some(x);
    }
    if let Some(b) = f.run_smoke {
        args.run_smoke = if b { "true".into() } else { "false".into() };
    }
    if let Some(b) = f.run_company_read {
        args.run_company_read = if b { "true".into() } else { "false".into() };
    }
    if let Some(b) = f.run_company_crud {
        args.run_company_crud = if b { "true".into() } else { "false".into() };
    }
    if let Some(x) = f.output_path {
        args.output_path = Some(x);
    }
    if let Some(x) = f.output_json {
        args.output_json = x;
    }
}

#[derive(Debug, Deserialize)]
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

fn load_config(repo_root: &str, path_opt: Option<&str>) -> ToolConfig {
    let default_rel = "scripts/tools/run-test-e2e-local/run-test-e2e-local-config.json";
    let path = path_opt.unwrap_or(default_rel);
    let mut p = std::path::PathBuf::from(repo_root);
    p.push(path);
    if !p.exists() {
        p = std::path::PathBuf::from(repo_root)
            .join("scripts/tools/run-test-e2e-local/run-test-e2e-local-config.json");
    }
    let Ok(s) = std::fs::read_to_string(&p) else {
        return ToolConfig {
            default_base_url: Some("http://localhost:5010".into()),
            demo_company_id: Some("11111111-1111-1111-1111-111111111115".into()),
            demo_company_name: Some("Empresa Demo".into()),
            http_timeout_seconds: Some(60),
        };
    };
    serde_json::from_str(&s).unwrap_or_else(|_| ToolConfig {
        default_base_url: Some("http://localhost:5010".into()),
        demo_company_id: Some("11111111-1111-1111-1111-111111111115".into()),
        demo_company_name: Some("Empresa Demo".into()),
        http_timeout_seconds: Some(60),
    })
}

fn repo_root() -> String {
    std::env::var("GESFER_REPO_ROOT").unwrap_or_else(|_| {
        std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| ".".into())
    })
}

fn admin_user(req: &CapsuleRequestFields) -> String {
    req.admin_user
        .clone()
        .or_else(|| std::env::var("E2E_ADMIN_USER").ok())
        .unwrap_or_else(|| "admin".into())
}

fn admin_password(req: &CapsuleRequestFields) -> String {
    req.admin_password
        .clone()
        .or_else(|| std::env::var("E2E_ADMIN_PASSWORD").ok())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "admin123".into())
}

fn internal_secret(req: &CapsuleRequestFields) -> String {
    req.internal_secret
        .clone()
        .or_else(|| std::env::var("E2E_INTERNAL_SECRET").ok())
        .unwrap_or_else(|| "dev-internal-secret-change-in-production".into())
}

fn http_client(timeout: Duration) -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| e.to_string())
}

fn emit(
    success: bool,
    exit_code: i32,
    message: &str,
    feedback: Vec<FeedbackEntry>,
    result: serde_json::Value,
    duration_ms: Option<u64>,
    args: &Args,
    agent_capsule: bool,
) -> ! {
    let res = CapsuleResponse::tool(
        TOOL_ID,
        success,
        exit_code,
        message,
        feedback,
        result,
        duration_ms,
    );
    let json = to_contract_json(&res).unwrap_or_else(|_| "{}".to_string());
    if agent_capsule
        || args.output_json
        || std::env::var("TOOLS_OUTPUT_JSON").as_deref() == Ok("1")
    {
        println!("{}", json);
    }
    if let Some(ref path) = args.output_path {
        let _ = std::fs::write(path, &json);
    }
    std::process::exit(exit_code);
}

fn main() {
    let start = Instant::now();
    let mut feedback = Vec::new();
    let mut args = Args::parse();
    let mut capsule_fields = CapsuleRequestFields::default();

    let agent_capsule = match gesfer_tools::try_read_capsule_request() {
        Ok(None) => false,
        Ok(Some(req)) => {
            if req.meta.entity_id != TOOL_ID {
                feedback.push(FeedbackEntry::warning(
                    "init",
                    &format!(
                        "entity_id en petición ({}) no coincide con {}; se continúa.",
                        req.meta.entity_id, TOOL_ID
                    ),
                    None,
                ));
            }
            merge_capsule(&mut args, &req.request);
            capsule_fields = serde_json::from_value(req.request).unwrap_or_default();
            true
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2);
        }
    };

    let repo_root = repo_root();
    let cfg = load_config(&repo_root, args.config_path.as_deref());
    let timeout_secs = cfg.http_timeout_seconds.unwrap_or(60);
    let timeout = Duration::from_secs(timeout_secs);

    let base = args
        .base_url
        .clone()
        .or(capsule_fields.base_url.clone())
        .or(cfg.default_base_url.clone())
        .unwrap_or_else(|| "http://localhost:5010".into());
    let base = base.trim_end_matches('/').to_string();

    let demo_id = cfg
        .demo_company_id
        .clone()
        .or(capsule_fields.demo_company_id.clone())
        .unwrap_or_else(|| "11111111-1111-1111-1111-111111111115".into());
    let demo_name = cfg
        .demo_company_name
        .clone()
        .or(capsule_fields.demo_company_name.clone())
        .unwrap_or_else(|| "Empresa Demo".into());

    let run_smoke = parse_bool_str(&args.run_smoke);
    let run_company_read = parse_bool_str(&args.run_company_read);
    let run_company_crud = parse_bool_str(&args.run_company_crud);

    let client = match http_client(timeout) {
        Ok(c) => c,
        Err(e) => {
            feedback.push(FeedbackEntry::error("init", "No se pudo crear cliente HTTP", Some(&e)));
            emit(
                false,
                3,
                "Cliente HTTP",
                feedback,
                json!({ "errorType": "http_client" }),
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }
    };

    let user = admin_user(&capsule_fields);
    let pass = admin_password(&capsule_fields);
    let secret = internal_secret(&capsule_fields);

    feedback.push(FeedbackEntry::info(
        "init",
        &format!("run-test-e2e-local baseUrl={} (sin credenciales en salida)", base),
    ));

    let mut result = json!({
        "baseUrl": base,
        "runSmoke": run_smoke,
        "runCompanyRead": run_company_read,
        "runCompanyCrud": run_company_crud
    });

    let mut token: Option<String> = None;

    if run_smoke {
        feedback.push(FeedbackEntry::info("smoke", "Comprobando /health y /swagger/v1/swagger.json"));
        let health = client.get(format!("{}/health", base)).send();
        let health_ok = match &health {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        };
        if !health_ok {
            feedback.push(FeedbackEntry::error("smoke", "GET /health no responde OK", None));
            result["smoke"] = json!({
                "healthOk": false,
                "swaggerOk": false,
                "loginOk": false,
                "loginInvalidUnauthorized": false
            });
            emit(
                false,
                10,
                "Smoke: health falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        let swagger = client
            .get(format!("{}/swagger/v1/swagger.json", base))
            .send();
        let swagger_ok = match swagger {
            Ok(r) => {
                if r.status().is_success() {
                    r.text()
                        .map(|t| t.trim_start().starts_with('{'))
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            Err(_) => false,
        };
        if !swagger_ok {
            feedback.push(FeedbackEntry::error("smoke", "GET swagger.json no válido", None));
            result["smoke"] = json!({
                "healthOk": true,
                "swaggerOk": false,
                "loginOk": false,
                "loginInvalidUnauthorized": false
            });
            emit(
                false,
                11,
                "Smoke: swagger falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        let login_body = json!({
            "usuario": user,
            "contraseña": pass
        });
        let login_resp = client
            .post(format!("{}/api/admin/auth/login", base))
            .json(&login_body)
            .send();
        let (login_ok, tok) = match login_resp {
            Ok(r) => {
                if r.status().is_success() {
                    let v: serde_json::Value = r.json().unwrap_or(json!({}));
                    let t = v.get("token").and_then(|x| x.as_str()).map(String::from);
                    (t.is_some(), t)
                } else {
                    (false, None)
                }
            }
            Err(_) => (false, None),
        };
        if !login_ok {
            feedback.push(FeedbackEntry::error("smoke", "Login admin no devolvió token", None));
            result["smoke"] = json!({
                "healthOk": true,
                "swaggerOk": true,
                "loginOk": false,
                "loginInvalidUnauthorized": false
            });
            emit(
                false,
                12,
                "Smoke: login falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }
        token = tok;

        let bad_login = json!({
            "usuario": user,
            "contraseña": "__invalid_password_e2e__"
        });
        let bad = client
            .post(format!("{}/api/admin/auth/login", base))
            .json(&bad_login)
            .send();
        let unauthorized = match bad {
            Ok(r) => r.status().as_u16() == 401,
            Err(_) => false,
        };
        if !unauthorized {
            feedback.push(FeedbackEntry::error(
                "smoke",
                "Login con credenciales inválidas no devolvió 401",
                None,
            ));
            result["smoke"] = json!({
                "healthOk": true,
                "swaggerOk": true,
                "loginOk": true,
                "loginInvalidUnauthorized": false
            });
            emit(
                false,
                13,
                "Smoke: 401 esperado",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        result["smoke"] = json!({
            "healthOk": true,
            "swaggerOk": true,
            "loginOk": true,
            "loginInvalidUnauthorized": true
        });
        feedback.push(FeedbackEntry::info("smoke", "Smoke OK"));
    }

    if run_company_read {
        if token.is_none() {
            let login_body = json!({ "usuario": user, "contraseña": pass });
            let login_resp = client
                .post(format!("{}/api/admin/auth/login", base))
                .json(&login_body)
                .send();
            token = match login_resp {
                Ok(r) if r.status().is_success() => r
                    .json::<serde_json::Value>()
                    .ok()
                    .and_then(|v| v.get("token").and_then(|x| x.as_str()).map(String::from)),
                _ => None,
            };
        }
        let Some(ref t) = token else {
            feedback.push(FeedbackEntry::error("company_read", "Sin token JWT", None));
            result["companyRead"] = json!({ "error": "no_token" });
            emit(
                false,
                20,
                "Company read: sin token",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        };

        feedback.push(FeedbackEntry::info("company_read", "GET /api/company (JWT y secret)"));
        let list_jwt = client
            .get(format!("{}/api/company", base))
            .header("Authorization", format!("Bearer {}", t))
            .send();
        let list_jwt_ok = match &list_jwt {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        };
        let mut empresa_demo_in_list = false;
        if list_jwt_ok {
            if let Ok(r) = list_jwt {
                if let Ok(arr) = r.json::<Vec<serde_json::Value>>() {
                    empresa_demo_in_list = arr.iter().any(|c| {
                        c.get("name").and_then(|x| x.as_str()) == Some(demo_name.as_str())
                    });
                }
            }
        }

        let list_secret = client
            .get(format!("{}/api/company", base))
            .header("X-Internal-Secret", &secret)
            .send();
        let list_secret_ok = match &list_secret {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        };

        let one = client
            .get(format!("{}/api/company/{}", base, demo_id))
            .header("Authorization", format!("Bearer {}", t))
            .send();
        let get_by_id_ok = match &one {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        };

        let no_auth = client.get(format!("{}/api/company", base)).send();
        let unauthorized_list = match no_auth {
            Ok(r) => r.status().as_u16() == 401,
            Err(_) => false,
        };

        if !list_jwt_ok
            || !empresa_demo_in_list
            || !list_secret_ok
            || !get_by_id_ok
            || !unauthorized_list
        {
            feedback.push(FeedbackEntry::error(
                "company_read",
                "Validación company read falló",
                None,
            ));
            result["companyRead"] = json!({
                "listJwtOk": list_jwt_ok,
                "empresaDemoInList": empresa_demo_in_list,
                "listSecretOk": list_secret_ok,
                "getByIdOk": get_by_id_ok,
                "unauthorizedWithoutAuth": unauthorized_list
            });
            emit(
                false,
                21,
                "Company read falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        result["companyRead"] = json!({
            "listJwtOk": true,
            "empresaDemoInList": true,
            "listSecretOk": true,
            "getByIdOk": true,
            "unauthorizedWithoutAuth": true
        });
        feedback.push(FeedbackEntry::info("company_read", "Company read OK"));
    }

    if run_company_crud {
        let login_body = json!({ "usuario": user, "contraseña": pass });
        let login_resp = client
            .post(format!("{}/api/admin/auth/login", base))
            .json(&login_body)
            .send();
        let t = match login_resp {
            Ok(r) if r.status().is_success() => {
                let v: serde_json::Value = r.json().unwrap_or(json!({}));
                v.get("token").and_then(|x| x.as_str()).map(String::from)
            }
            _ => None,
        };
        let Some(t) = t else {
            feedback.push(FeedbackEntry::error("company_crud", "Login para CRUD falló", None));
            result["companyCrud"] = json!({ "ok": false, "error": "no_token" });
            emit(
                false,
                30,
                "Company CRUD: sin token",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        };

        let suffix = Uuid::new_v4().simple().to_string();
        let name_create = format!("E2E Tool {}", suffix);
        let name_update = format!("E2E Tool Mod {}", suffix);

        feedback.push(FeedbackEntry::info("company_crud", "POST crear empresa"));
        let create_payload = json!({
            "name": name_create,
            "taxId": "B12345674",
            "address": "Calle E2E Creación 1",
            "phone": "910000001",
            "email": format!("e2e-{}@local.test", suffix)
        });
        let create_res = client
            .post(format!("{}/api/company", base))
            .header("Authorization", format!("Bearer {}", t))
            .json(&create_payload)
            .send();

        let (create_ok, created_id) = match create_res {
            Ok(r) => {
                let st = r.status();
                if st.as_u16() == 201 {
                    let v: serde_json::Value = r.json().unwrap_or(json!({}));
                    let id = v
                        .get("id")
                        .and_then(|x| x.as_str())
                        .map(String::from)
                        .unwrap_or_default();
                    (!id.is_empty(), id)
                } else {
                    (false, String::new())
                }
            }
            Err(_) => (false, String::new()),
        };
        if !create_ok {
            feedback.push(FeedbackEntry::error("company_crud", "POST crear empresa falló", None));
            result["companyCrud"] = json!({ "ok": false, "step": "create" });
            emit(
                false,
                31,
                "CRUD: crear falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        feedback.push(FeedbackEntry::info("company_crud", "PUT actualizar empresa"));
        let update_payload = json!({
            "name": name_update,
            "taxId": "B87654315",
            "address": "Calle E2E Modificación 99",
            "phone": "920000002",
            "email": format!("e2e-upd-{}@local.test", suffix),
            "isActive": true
        });
        let upd = client
            .put(format!("{}/api/company/{}", base, created_id))
            .header("Authorization", format!("Bearer {}", t))
            .json(&update_payload)
            .send();
        let update_ok = match upd {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        };
        if !update_ok {
            feedback.push(FeedbackEntry::error("company_crud", "PUT actualizar falló", None));
            result["companyCrud"] = json!({
                "ok": false,
                "step": "update",
                "companyId": created_id
            });
            emit(
                false,
                32,
                "CRUD: actualizar falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        feedback.push(FeedbackEntry::info("company_crud", "DELETE empresa"));
        let del = client
            .delete(format!("{}/api/company/{}", base, created_id))
            .header("Authorization", format!("Bearer {}", t))
            .send();
        let delete_ok = match del {
            Ok(r) => r.status().as_u16() == 204,
            Err(_) => false,
        };
        if !delete_ok {
            feedback.push(FeedbackEntry::error("company_crud", "DELETE falló", None));
            result["companyCrud"] = json!({
                "ok": false,
                "step": "delete",
                "companyId": created_id
            });
            emit(
                false,
                33,
                "CRUD: eliminar falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        let get_after = client
            .get(format!("{}/api/company/{}", base, created_id))
            .header("Authorization", format!("Bearer {}", t))
            .send();
        let not_found_after = match get_after {
            Ok(r) => r.status().as_u16() == 404,
            Err(_) => false,
        };
        if !not_found_after {
            feedback.push(FeedbackEntry::error(
                "company_crud",
                "GET tras borrar no devolvió 404",
                None,
            ));
            result["companyCrud"] = json!({
                "ok": false,
                "step": "verify404",
                "companyId": created_id
            });
            emit(
                false,
                34,
                "CRUD: verificación 404 falló",
                feedback,
                result,
                Some(start.elapsed().as_millis() as u64),
                &args,
                agent_capsule,
            );
        }

        result["companyCrud"] = json!({
            "ok": true,
            "companyId": created_id,
            "steps": ["create", "update", "delete", "verify404"]
        });
        feedback.push(FeedbackEntry::info("company_crud", "Company CRUD OK"));
    }

    feedback.push(FeedbackEntry::info(
        "done",
        "Todas las pruebas E2E locales completadas",
    ));
    emit(
        true,
        0,
        "E2E local OK",
        feedback,
        result,
        Some(start.elapsed().as_millis() as u64),
        &args,
        agent_capsule,
    );
}
