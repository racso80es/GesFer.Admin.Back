//! Skill git-sync-remote — fetch + pull --rebase + push (opcional --force-with-lease).

use std::process::Command;

use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "git-sync-remote";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SyncRequest {
    #[serde(default)]
    force: Option<bool>,
}

#[derive(Debug, Parser)]
#[command(name = "git_sync_remote", version, about = "Git sync: fetch + pull --rebase + push")]
struct CliArgs {
    #[arg(long)]
    force: bool,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: SyncRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let force = body.force.unwrap_or(false);
        let (res, exit_code) = run(force, start);
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    let args = CliArgs::parse();
    let (res, exit_code) = run(args.force, start);
    let _ = write_capsule_response(&res);
    std::process::exit(exit_code);
}

fn run(force: bool, start: std::time::Instant) -> (CapsuleResponse, i32) {
    let mut feedback: Vec<FeedbackEntry> = vec![];

    let fetch = run_git(&["fetch"]);
    feedback.push(if fetch.exit_code == 0 {
        FeedbackEntry::info("git", "git fetch")
    } else {
        FeedbackEntry::error("git", "git fetch falló", Some(&fetch.combined))
    });
    if fetch.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            fetch.exit_code,
            "Fetch falló",
            feedback,
            serde_json::json!({
                "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
                "pullRebase": { "exitCode": null, "output": "" },
                "push": { "exitCode": null, "output": "" }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, fetch.exit_code);
    }

    let pull = run_git(&["pull", "--rebase"]);
    feedback.push(if pull.exit_code == 0 {
        FeedbackEntry::info("git", "git pull --rebase")
    } else {
        FeedbackEntry::error("git", "git pull --rebase falló", Some(&pull.combined))
    });
    if pull.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            pull.exit_code,
            "Pull --rebase falló",
            feedback,
            serde_json::json!({
                "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
                "pullRebase": { "exitCode": pull.exit_code, "output": pull.combined },
                "push": { "exitCode": null, "output": "" }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, pull.exit_code);
    }

    let push = if force {
        run_git(&["push", "origin", "HEAD", "--force-with-lease"])
    } else {
        run_git(&["push", "origin", "HEAD"])
    };
    let push_lower = push.combined.to_lowercase();
    let non_critical_push = push.exit_code == 0
        && (push_lower.contains("everything up-to-date") || push_lower.contains("already up to date"));

    if push.exit_code != 0 {
        feedback.push(FeedbackEntry::error("git", "git push falló", Some(&push.combined)));
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            push.exit_code,
            "Push falló",
            feedback,
            serde_json::json!({
                "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
                "pullRebase": { "exitCode": pull.exit_code, "output": pull.combined },
                "push": { "exitCode": push.exit_code, "output": push.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, push.exit_code);
    }

    if non_critical_push {
        feedback.push(FeedbackEntry::warning("git", "Everything up-to-date", None));
    } else {
        feedback.push(FeedbackEntry::info("git", "Push completado"));
    }

    let res = CapsuleResponse::skill(
        SKILL_ID,
        true,
        0,
        if non_critical_push {
            "Sin cambios para sincronizar"
        } else {
            "Sincronización completada"
        },
        feedback,
        serde_json::json!({
            "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
            "pullRebase": { "exitCode": pull.exit_code, "output": pull.combined },
            "push": { "exitCode": push.exit_code, "output": push.combined }
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

