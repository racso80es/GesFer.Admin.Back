//! Skill git-branch-manager — git switch / git switch -c con salida JSON v2.

use std::process::Command;

use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "git-branch-manager";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct BranchRequest {
    #[serde(default)]
    branch_name: Option<String>,
    #[serde(default)]
    create: Option<bool>,
}

#[derive(Debug, Parser)]
#[command(name = "git_branch_manager", version, about = "Cambia o crea ramas con git switch")]
struct CliArgs {
    #[arg(long = "branch-name")]
    branch_name: String,
    #[arg(long)]
    create: bool,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: BranchRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let branch = body.branch_name.unwrap_or_default();
        let create = body.create.unwrap_or(false);
        if branch.trim().is_empty() {
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                1,
                "request.branch_name obligatorio",
                vec![FeedbackEntry::error("validate", "branch_name vacío", None)],
                serde_json::json!({}),
                Some(start.elapsed().as_millis() as u64),
            );
            let _ = write_capsule_response(&res);
            std::process::exit(1);
        }
        let (res, exit_code) = run(branch, create, start);
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    // CLI (humano)
    let args = CliArgs::parse();
    let (res, exit_code) = run(args.branch_name, args.create, start);
    let _ = write_capsule_response(&res);
    std::process::exit(exit_code);
}

fn run(branch_name: String, create: bool, start: std::time::Instant) -> (CapsuleResponse, i32) {
    let mut feedback = vec![FeedbackEntry::info(
        "git",
        if create { "git switch -c" } else { "git switch" },
    )];

    let mut cmd = Command::new("git");
    if create {
        cmd.args(["switch", "-c", branch_name.trim()]);
    } else {
        cmd.args(["switch", branch_name.trim()]);
    }
    let switch_out = cmd.output();

    let (switch_exit, switch_output, switch_ok) = match switch_out {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let code = o.status.code().unwrap_or(-1);
            (code, format!("{}\n{}", out, err).trim().to_string(), o.status.success())
        }
        Err(e) => (1, e.to_string(), false),
    };

    if !switch_ok {
        feedback.push(FeedbackEntry::error("git", "git switch falló", Some(&switch_output)));
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            switch_exit,
            "No se pudo cambiar/crear la rama",
            feedback,
            serde_json::json!({
                "switch": { "exitCode": switch_exit, "output": switch_output }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, switch_exit);
    }

    let active = active_branch().unwrap_or_else(|| "unknown".to_string());
    feedback.push(FeedbackEntry::info("git", "Rama activa resuelta"));
    let res = CapsuleResponse::skill(
        SKILL_ID,
        true,
        0,
        "Rama activa actualizada",
        feedback,
        serde_json::json!({
            "activeBranch": active,
            "switch": { "exitCode": switch_exit, "output": switch_output }
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, 0)
}

fn active_branch() -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}

