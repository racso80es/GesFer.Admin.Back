use std::fs;

use clap::Parser;
use gesfer_tools::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const TOOL_ID: &str = "run-test-e2e-local";

#[derive(Debug, Parser)]
#[command(name = "run_test_e2e_local")]
struct Cli {
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    output_json: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct JsonRequest {
    #[serde(default)]
    output_path: Option<String>,
    #[serde(default)]
    output_json: Option<bool>,
}

fn main() {
    let started = std::time::Instant::now();
    let mut feedback = vec![FeedbackEntry::info("init", "Iniciando run-test-e2e-local (Rust)")];

    let capsule_req = try_read_capsule_request().ok().flatten();
    let mut cli = Cli::parse();
    if let Some(req) = capsule_req {
        if let Ok(j) = serde_json::from_value::<JsonRequest>(req.request) {
            if j.output_path.is_some() { cli.output_path = j.output_path; }
            if let Some(v) = j.output_json { cli.output_json = v; }
        }
    }

    feedback.push(FeedbackEntry::warning("e2e", "Tool no implementada (placeholder compilable).", None));
    let res = CapsuleResponse::tool(
        TOOL_ID,
        false,
        2,
        "E2E no implementado",
        feedback,
        serde_json::json!({}),
        Some(started.elapsed().as_millis() as u64),
    );

    if let Some(p) = cli.output_path {
        let _ = fs::write(p, serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string()));
    }
    if cli.output_json {
        let _ = write_capsule_response(&res);
    }
    std::process::exit(res.exit_code);
}

