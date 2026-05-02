//! Skill git-sync-remote — fetch; pull --rebase solo si hay upstream; push (-u si no hay upstream).

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

/// Rama actual tiene upstream configurado (`@{u}` resuelve).
fn has_upstream() -> bool {
    let o = run_git(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"]);
    o.exit_code == 0
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
        let msg = format!(
            "Fetch falló (exit {}): {}",
            fetch.exit_code,
            summarize_output(&fetch.combined)
        );
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            fetch.exit_code,
            &msg,
            feedback,
            serde_json::json!({
                "hadUpstream": null,
                "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
                "pullRebase": { "skipped": true, "exitCode": null, "output": "" },
                "push": { "exitCode": null, "output": "" }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, fetch.exit_code);
    }

    let upstream = has_upstream();
    let pull = if upstream {
        let p = run_git(&["pull", "--rebase"]);
        feedback.push(if p.exit_code == 0 {
            FeedbackEntry::info("git", "git pull --rebase")
        } else {
            FeedbackEntry::error("git", "git pull --rebase falló", Some(&p.combined))
        });
        if p.exit_code != 0 {
            let msg = format!(
                "Pull --rebase falló (exit {}): {}",
                p.exit_code,
                summarize_output(&p.combined)
            );
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                p.exit_code,
                &msg,
                feedback,
                serde_json::json!({
                    "hadUpstream": true,
                    "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
                    "pullRebase": { "skipped": false, "exitCode": p.exit_code, "output": p.combined },
                    "push": { "exitCode": null, "output": "" }
                }),
                Some(start.elapsed().as_millis() as u64),
            );
            return (res, p.exit_code);
        }
        p
    } else {
        feedback.push(FeedbackEntry::info(
            "git",
            "Sin rama de seguimiento (upstream): se omite pull --rebase; push usará -u origin HEAD",
        ));
        CmdOut {
            exit_code: 0,
            combined: String::new(),
        }
    };

    let push = if upstream {
        if force {
            run_git(&["push", "origin", "HEAD", "--force-with-lease"])
        } else {
            run_git(&["push", "origin", "HEAD"])
        }
    } else if force {
        run_git(&["push", "-u", "origin", "HEAD", "--force-with-lease"])
    } else {
        run_git(&["push", "-u", "origin", "HEAD"])
    };

    feedback.push(if push.exit_code == 0 {
        FeedbackEntry::info("git", if upstream { "git push" } else { "git push -u origin HEAD" })
    } else {
        FeedbackEntry::error("git", "git push falló", Some(&push.combined))
    });

    if push.exit_code != 0 {
        let msg = format!(
            "Push falló (exit {}): {}",
            push.exit_code,
            summarize_output(&push.combined)
        );
        let pull_exit_json = if upstream {
            serde_json::to_value(pull.exit_code).unwrap_or(serde_json::Value::Null)
        } else {
            serde_json::Value::Null
        };
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            push.exit_code,
            &msg,
            feedback,
            serde_json::json!({
                "hadUpstream": upstream,
                "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
                "pullRebase": {
                    "skipped": !upstream,
                    "exitCode": pull_exit_json,
                    "output": pull.combined
                },
                "push": { "exitCode": push.exit_code, "output": push.combined },
                "pushMode": if upstream { "normal" } else { "setUpstream" }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, push.exit_code);
    }

    let push_lower = push.combined.to_lowercase();
    let non_critical_push = push_lower.contains("everything up-to-date")
        || push_lower.contains("already up to date");

    if non_critical_push {
        feedback.push(FeedbackEntry::warning("git", "Everything up-to-date", None));
    } else {
        feedback.push(FeedbackEntry::info("git", "Push completado"));
    }

    let pull_exit_ok = if upstream {
        serde_json::to_value(pull.exit_code).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::Value::Null
    };

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
            "hadUpstream": upstream,
            "fetch": { "exitCode": fetch.exit_code, "output": fetch.combined },
            "pullRebase": {
                "skipped": !upstream,
                "exitCode": pull_exit_ok,
                "output": pull.combined
            },
            "push": { "exitCode": push.exit_code, "output": push.combined },
            "pushMode": if upstream { "normal" } else { "setUpstream" }
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, 0)
}

fn summarize_output(s: &str) -> String {
    let t = s.trim();
    if t.len() <= 500 {
        t.to_string()
    } else {
        format!("{}…", &t[..500])
    }
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
