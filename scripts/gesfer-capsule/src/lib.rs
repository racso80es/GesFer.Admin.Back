//! Envelope JSON compartido para skills y tools (contrato v2, SddIA/norms/capsule-json-io.md).

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Nivel de una entrada de feedback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FeedbackLevel {
    Info,
    Warning,
    Error,
}

/// Entrada de feedback (fase, nivel, mensaje, timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEntry {
    pub phase: String,
    #[serde(rename = "level")]
    pub level: FeedbackLevel,
    pub message: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "duration_ms")]
    pub duration_ms: Option<u64>,
}

impl FeedbackEntry {
    pub fn info(phase: &str, message: &str) -> Self {
        Self {
            phase: phase.to_string(),
            level: FeedbackLevel::Info,
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            detail: None,
            duration_ms: None,
        }
    }

    pub fn warning(phase: &str, message: &str, detail: Option<&str>) -> Self {
        Self {
            phase: phase.to_string(),
            level: FeedbackLevel::Warning,
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            detail: detail.map(String::from),
            duration_ms: None,
        }
    }

    pub fn error(phase: &str, message: &str, detail: Option<&str>) -> Self {
        Self {
            phase: phase.to_string(),
            level: FeedbackLevel::Error,
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            detail: detail.map(String::from),
            duration_ms: None,
        }
    }
}

/// Petición por stdin (agente).
#[derive(Debug, Clone, Deserialize)]
pub struct CapsuleRequest {
    pub meta: RequestMeta,
    pub request: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestMeta {
    pub schema_version: String,
    pub entity_kind: String,
    pub entity_id: String,
    #[serde(default)]
    pub token: Option<String>,
}

/// Respuesta por stdout (contrato v2).
#[derive(Debug, Clone, Serialize)]
pub struct CapsuleResponse {
    pub meta: ResponseMeta,
    pub success: bool,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    pub message: String,
    pub feedback: Vec<FeedbackEntry>,
    pub result: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseMeta {
    pub schema_version: String,
    pub entity_kind: String,
    pub entity_id: String,
    pub timestamp: String,
}

impl CapsuleResponse {
    pub fn skill(
        entity_id: &str,
        success: bool,
        exit_code: i32,
        message: &str,
        feedback: Vec<FeedbackEntry>,
        result: serde_json::Value,
        duration_ms: Option<u64>,
    ) -> Self {
        Self {
            meta: ResponseMeta {
                schema_version: "2.0".to_string(),
                entity_kind: "skill".to_string(),
                entity_id: entity_id.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
            success,
            exit_code,
            message: message.to_string(),
            feedback,
            result,
            duration_ms,
        }
    }

    pub fn tool(
        entity_id: &str,
        success: bool,
        exit_code: i32,
        message: &str,
        feedback: Vec<FeedbackEntry>,
        result: serde_json::Value,
        duration_ms: Option<u64>,
    ) -> Self {
        Self {
            meta: ResponseMeta {
                schema_version: "2.0".to_string(),
                entity_kind: "tool".to_string(),
                entity_id: entity_id.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
            success,
            exit_code,
            message: message.to_string(),
            feedback,
            result,
            duration_ms,
        }
    }
}

/// Variable de entorno: envelope JSON (una línea) sin usar stdin. Útil cuando el runner no puede cerrar stdin de forma fiable.
pub const ENV_CAPSULE_REQUEST: &str = "GESFER_CAPSULE_REQUEST";

/// Variable de entorno: si es `1` o `true`, no se lee stdin y se asume modo CLI (mismos args que con TTY).
/// Evita bloqueos cuando stdin no es TTY pero el padre deja el pipe abierto sin datos ni EOF.
pub const ENV_SKIP_STDIN: &str = "GESFER_SKIP_STDIN";

fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Resuelve la petición capsule-json-io con este orden:
/// 1. [`ENV_CAPSULE_REQUEST`]: JSON del envelope completo; **no** lee stdin.
/// 2. [`ENV_SKIP_STDIN`]: modo CLI sin leer stdin (`Ok(None)`).
/// 3. stdin es TTY: modo CLI (`Ok(None)`).
/// 4. stdin no TTY: lee hasta EOF (puede bloquear si el padre no cierra stdin).
///
/// `None` si modo CLI o stdin vacío tras EOF. `Some(request)` si hay JSON válido.
pub fn try_read_capsule_request() -> Result<Option<CapsuleRequest>, String> {
    if let Ok(raw) = std::env::var(ENV_CAPSULE_REQUEST) {
        let buf = raw.trim();
        if !buf.is_empty() {
            return serde_json::from_str(buf)
                .map_err(|e| format!("JSON de entrada inválido ({}): {}", ENV_CAPSULE_REQUEST, e))
                .map(Some);
        }
    }
    if env_truthy(ENV_SKIP_STDIN) {
        return Ok(None);
    }
    if atty::is(atty::Stream::Stdin) {
        return Ok(None);
    }
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).map_err(|e| e.to_string())?;
    let buf = buf.trim();
    if buf.is_empty() {
        // En lugar de fallar, asumimos que no hay petición JSON y operamos en modo CLI / CI
        return Ok(None);
    }
    serde_json::from_str(buf)
        .map_err(|e| format!("JSON de entrada inválido: {}", e))
        .map(Some)
}

pub fn write_capsule_response(res: &CapsuleResponse) -> Result<(), String> {
    let s = serde_json::to_string(res).map_err(|e| e.to_string())?;
    println!("{}", s);
    Ok(())
}

pub fn exit_process_code(res: &CapsuleResponse) -> ! {
    std::process::exit(res.exit_code);
}
