//! Biblioteca del contrato de herramientas GesFer (v2: envelope capsule-json-io).
//! Reexporta tipos del crate `gesfer-capsule` y helpers de serialización.

pub use gesfer_capsule::{
    try_read_capsule_request, write_capsule_response, CapsuleRequest, CapsuleResponse, FeedbackEntry,
    FeedbackLevel, RequestMeta, ResponseMeta, ENV_CAPSULE_REQUEST, ENV_SKIP_STDIN,
};

use serde::{Deserialize, Serialize};

/// Compatibilidad: resultado plano v1 (deprecated). Preferir [`CapsuleResponse`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    pub tool_id: String,
    pub exit_code: i32,
    pub success: bool,
    pub timestamp: String,
    pub message: String,
    pub feedback: Vec<FeedbackEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "duration_ms")]
    pub duration_ms: Option<u64>,
}

impl ToolResult {
    pub fn to_capsule_response(self) -> CapsuleResponse {
        CapsuleResponse::tool(
            &self.tool_id,
            self.success,
            self.exit_code,
            &self.message,
            self.feedback,
            self.result.unwrap_or_else(|| serde_json::json!({})),
            self.duration_ms,
        )
    }
}

/// Serializa respuesta v2 (envelope completo).
pub fn to_contract_json(result: &CapsuleResponse) -> Result<String, serde_json::Error> {
    serde_json::to_string(result)
}
