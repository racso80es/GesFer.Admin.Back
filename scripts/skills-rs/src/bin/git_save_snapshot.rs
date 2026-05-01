//! Skill git-save-snapshot — git add . + git commit -m, tolerando "nothing to commit".

use std::process::Command;

use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "git-save-snapshot";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SnapshotRequest {
    #[serde(default)]
    commit_message: Option<String>,
}

#[derive(Debug, Parser)]
#[command(name = "git_save_snapshot", version, about = "Git snapshot: add . + commit -m")]
struct CliArgs {
    #[arg(long = "commit-message")]
    commit_message: String,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: SnapshotRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let msg = body.commit_message.unwrap_or_default();
        if msg.trim().is_empty() {
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                1,
                "request.commit_message obligatorio",
                vec![FeedbackEntry::error("validate", "commit_message vacío", None)],
                serde_json::json!({}),
                Some(start.elapsed().as_millis() as u64),
            );
            let _ = write_capsule_response(&res);
            std::process::exit(1);
        }
        let (res, exit_code) = run(msg, start);
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    // CLI (humano)
    let args = CliArgs::parse();
    let (res, exit_code) = run(args.commit_message, start);
    let _ = write_capsule_response(&res);
    std::process::exit(exit_code);
}

fn run(commit_message: String, start: std::time::Instant) -> (CapsuleResponse, i32) {
    let mut feedback: Vec<FeedbackEntry> = vec![];

    let add_out = run_git(&["add", "."]);
    feedback.push(if add_out.exit_code == 0 {
        FeedbackEntry::info("git", "git add .")
    } else {
        FeedbackEntry::error("git", "git add . falló", Some(&add_out.combined))
    });
    if add_out.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            add_out.exit_code,
            "git add falló",
            feedback,
            serde_json::json!({
                "committed": false,
                "commitHash": null,
                "add": { "exitCode": add_out.exit_code, "output": add_out.combined },
                "commit": { "exitCode": null, "output": "" }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, add_out.exit_code);
    }

    let commit_out = run_git(&["commit", "-m", commit_message.trim()]);
    let lower = commit_out.combined.to_lowercase();
    let nothing_to_commit = commit_out.exit_code != 0
        && (lower.contains("nothing to commit") || lower.contains("no changes added to commit"));

    if commit_out.exit_code != 0 && !nothing_to_commit {
        feedback.push(FeedbackEntry::error(
            "git",
            "git commit falló",
            Some(&commit_out.combined),
        ));
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            commit_out.exit_code,
            "git commit falló",
            feedback,
            serde_json::json!({
                "committed": false,
                "commitHash": null,
                "add": { "exitCode": add_out.exit_code, "output": add_out.combined },
                "commit": { "exitCode": commit_out.exit_code, "output": commit_out.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, commit_out.exit_code);
    }

    if nothing_to_commit {
        feedback.push(FeedbackEntry::warning("git", "Nothing to commit", None));
        let res = CapsuleResponse::skill(
            SKILL_ID,
            true,
            0,
            "Nothing to commit",
            feedback,
            serde_json::json!({
                "committed": false,
                "commitHash": null,
                "add": { "exitCode": add_out.exit_code, "output": add_out.combined },
                "commit": { "exitCode": commit_out.exit_code, "output": commit_out.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, 0);
    }

    feedback.push(FeedbackEntry::info("git", "Commit creado"));
    let hash = run_git(&["rev-parse", "HEAD"]);
    let commit_hash = if hash.exit_code == 0 {
        hash.combined.lines().next().unwrap_or("").trim().to_string()
    } else {
        "".to_string()
    };

    let res = CapsuleResponse::skill(
        SKILL_ID,
        true,
        0,
        "Snapshot guardado",
        feedback,
        serde_json::json!({
            "committed": true,
            "commitHash": if commit_hash.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(commit_hash) },
            "add": { "exitCode": add_out.exit_code, "output": add_out.combined },
            "commit": { "exitCode": commit_out.exit_code, "output": commit_out.combined }
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, 0)
}

struct CmdOut {
    exit_code: i32,
    combined: String,
}

fn run_git(args: &[&str]) -> CmdOut {
    let output = Command::new("git").args(args).output();
    match output {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let code = o.status.code().unwrap_or(-1);
            CmdOut {
                exit_code: code,
                combined: format!("{}\n{}", out, err).trim().to_string(),
            }
        }
        Err(e) => CmdOut {
            exit_code: 1,
            combined: e.to_string(),
        },
    }
}

