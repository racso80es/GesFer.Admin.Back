//! Skill git-close-cycle — cierre local post-fusión: troncal, pull, fetch --prune, borrar rama de trabajo.

use std::process::Command;

use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;
use serde_json::json;

const SKILL_ID: &str = "git-close-cycle";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CloseCycleRequest {
    #[serde(default)]
    target_branch: Option<String>,
}

#[derive(Debug, Parser)]
#[command(name = "git_close_cycle", version, about = "Cierra ciclo Git local (troncal + borrar rama)")]
struct CliArgs {
    #[arg(long = "target-branch")]
    target_branch: String,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: CloseCycleRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let target = body.target_branch.unwrap_or_default();
        let (res, code) = run(&target, start);
        let _ = write_capsule_response(&res);
        std::process::exit(code);
    }

    let args = CliArgs::parse();
    let (res, code) = run(&args.target_branch, start);
    let _ = write_capsule_response(&res);
    std::process::exit(code);
}

fn run(target_raw: &str, start: std::time::Instant) -> (CapsuleResponse, i32) {
    let target = target_raw.trim();
    if target.is_empty() {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            1,
            "request.targetBranch obligatorio",
            vec![FeedbackEntry::error("validate", "targetBranch vacío", None)],
            json!({}),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, 1);
    }

    let Some(trunk) = resolve_trunk() else {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            2,
            "No se encontró rama troncal local (main ni master)",
            vec![FeedbackEntry::error(
                "git",
                "Falta refs/heads/main o refs/heads/master",
                None,
            )],
            json!({ "trunk": null }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, 2);
    };

    if target == trunk.as_str() {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            3,
            "targetBranch no puede ser el troncal",
            vec![FeedbackEntry::error(
                "validate",
                "No eliminar la rama troncal",
                Some(trunk.as_str()),
            )],
            json!({ "trunk": trunk }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, 3);
    }

    let mut feedback: Vec<FeedbackEntry> = vec![];

    let co = run_git(&["checkout", trunk.as_str()]);
    feedback.push(if co.exit_code == 0 {
        FeedbackEntry::info("git", "git checkout troncal")
    } else {
        FeedbackEntry::error("git", "git checkout falló", Some(&co.combined))
    });
    if co.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            co.exit_code,
            "Checkout del troncal falló",
            feedback,
            json!({
                "trunk": trunk,
                "checkout": { "exitCode": co.exit_code, "output": co.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, co.exit_code);
    }

    let pull = run_git(&["pull", "origin", "HEAD"]);
    feedback.push(if pull.exit_code == 0 {
        FeedbackEntry::info("git", "git pull origin HEAD")
    } else {
        FeedbackEntry::error("git", "git pull origin HEAD falló", Some(&pull.combined))
    });
    if pull.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            pull.exit_code,
            "Pull del troncal falló",
            feedback,
            json!({
                "trunk": trunk,
                "checkout": { "exitCode": co.exit_code, "output": co.combined },
                "pull": { "exitCode": pull.exit_code, "output": pull.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, pull.exit_code);
    }

    let fetch = run_git(&["fetch", "--prune"]);
    feedback.push(if fetch.exit_code == 0 {
        FeedbackEntry::info("git", "git fetch --prune")
    } else {
        FeedbackEntry::error("git", "git fetch --prune falló", Some(&fetch.combined))
    });
    if fetch.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            fetch.exit_code,
            "fetch --prune falló",
            feedback,
            json!({
                "trunk": trunk,
                "checkout": { "exitCode": co.exit_code, "output": co.combined },
                "pull": { "exitCode": pull.exit_code, "output": pull.combined },
                "fetchPrune": { "exitCode": fetch.exit_code, "output": fetch.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, fetch.exit_code);
    }

    let delete_result = if local_branch_exists(target) {
        let d = run_git(&["branch", "-d", target]);
        if d.exit_code == 0 {
            feedback.push(FeedbackEntry::info("git", "git branch -d"));
            json!({
                "mode": "delete",
                "exitCode": d.exit_code,
                "output": d.combined,
                "usedForce": false
            })
        } else {
            let dd = run_git(&["branch", "-D", target]);
            feedback.push(if dd.exit_code == 0 {
                FeedbackEntry::warning("git", "git branch -d falló; aplicado -D", Some(&d.combined))
            } else {
                FeedbackEntry::error("git", "git branch -D falló", Some(&dd.combined))
            });
            json!({
                "mode": "forceDelete",
                "softAttempt": { "exitCode": d.exit_code, "output": d.combined },
                "exitCode": dd.exit_code,
                "output": dd.combined,
                "usedForce": dd.exit_code == 0
            })
        }
    } else {
        feedback.push(FeedbackEntry::warning(
            "git",
            "Rama objetivo inexistente en local; omitido branch -d",
            Some(target),
        ));
        json!({ "mode": "skipped", "reason": "branch not found locally" })
    };

    let delete_ok = match delete_result.get("mode").and_then(|m| m.as_str()) {
        Some("skipped") => true,
        _ => delete_result
            .get("exitCode")
            .and_then(|v| v.as_i64())
            .is_some_and(|c| c == 0),
    };

    if !delete_ok {
        let exit = delete_result
            .get("exitCode")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            exit,
            "No se pudo eliminar la rama de trabajo",
            feedback,
            json!({
                "trunk": trunk,
                "targetBranch": target,
                "checkout": { "exitCode": co.exit_code, "output": co.combined },
                "pull": { "exitCode": pull.exit_code, "output": pull.combined },
                "fetchPrune": { "exitCode": fetch.exit_code, "output": fetch.combined },
                "deleteBranch": delete_result
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, exit);
    }

    feedback.push(FeedbackEntry::info("git", "Ciclo local cerrado"));
    let res = CapsuleResponse::skill(
        SKILL_ID,
        true,
        0,
        "Troncal actualizado y rama de trabajo eliminada o ya ausente",
        feedback,
        json!({
            "trunk": trunk,
            "targetBranch": target,
            "checkout": { "exitCode": co.exit_code, "output": co.combined },
            "pull": { "exitCode": pull.exit_code, "output": pull.combined },
            "fetchPrune": { "exitCode": fetch.exit_code, "output": fetch.combined },
            "deleteBranch": delete_result
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, 0)
}

fn local_branch_exists(name: &str) -> bool {
    run_git(&["show-ref", "--verify", "--quiet", &format!("refs/heads/{name}")]).exit_code == 0
}

fn resolve_trunk() -> Option<String> {
    if local_branch_exists("main") {
        return Some("main".to_string());
    }
    if local_branch_exists("master") {
        return Some("master".to_string());
    }
    None
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
