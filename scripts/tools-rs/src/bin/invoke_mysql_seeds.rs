use gesfer_capsule::{write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde_json::json;

fn main() {
    let feedback = vec![
        FeedbackEntry::info("init", "invoke-mysql-seeds no implementado en este snapshot"),
        FeedbackEntry::error("error", "Tool no implementada", Some("Falta implementar src/bin/invoke_mysql_seeds.rs")),
    ];
    let res = CapsuleResponse::tool(
        "invoke-mysql-seeds",
        false,
        2,
        "NOT_IMPLEMENTED",
        feedback,
        json!({}),
        None,
    );
    let _ = write_capsule_response(&res);
    std::process::exit(res.exit_code);
}

