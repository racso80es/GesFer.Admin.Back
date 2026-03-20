//! Skill finalizar-git (post_pr) — TTY o JSON stdin.

use std::process::Command;
use std::{env, time::Instant};

use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "finalizar-git";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Body {
    #[serde(default)]
    branch_name: Option<String>,
    #[serde(default)]
    delete_remote: Option<bool>,
}

fn main() {
    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: Body = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let start = Instant::now();
        let delete_remote = body.delete_remote.unwrap_or(false);
        let branch_name = body.branch_name.clone().filter(|s| !s.is_empty());

        match run_cleanup(branch_name.as_deref(), delete_remote) {
            Ok(msg) => {
                let res = CapsuleResponse::skill(
                    SKILL_ID,
                    true,
                    0,
                    &msg,
                    vec![FeedbackEntry::info("git", &msg)],
                    serde_json::json!({ "branch": branch_name }),
                    Some(start.elapsed().as_millis() as u64),
                );
                let _ = write_capsule_response(&res);
                std::process::exit(0);
            }
            Err((code, msg)) => {
                let res = CapsuleResponse::skill(
                    SKILL_ID,
                    false,
                    code,
                    &msg,
                    vec![FeedbackEntry::error("git", &msg, None)],
                    serde_json::json!({}),
                    Some(start.elapsed().as_millis() as u64),
                );
                let _ = write_capsule_response(&res);
                std::process::exit(code);
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    let mut branch_name = String::new();
    let mut delete_remote = false;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--branch" && i + 1 < args.len() {
            branch_name = args[i + 1].clone();
            i += 2;
            continue;
        }
        if args[i] == "--delete-remote" {
            delete_remote = true;
        }
        i += 1;
    }
    let branch_opt = if branch_name.is_empty() {
        None
    } else {
        Some(branch_name.as_str())
    };
    match run_cleanup(branch_opt, delete_remote) {
        Ok(_) => std::process::exit(0),
        Err((code, _)) => std::process::exit(code),
    }
}

fn run_cleanup(branch_name_in: Option<&str>, delete_remote: bool) -> Result<String, (i32, String)> {
    let mut branch_name = branch_name_in.map(|s| s.to_string()).unwrap_or_default();
    if branch_name.is_empty() {
        if let Ok(out) = Command::new("git").args(["branch", "--show-current"]).output() {
            if out.status.success() {
                branch_name = String::from_utf8_lossy(&out.stdout).trim().to_string();
            }
        }
    }
    if branch_name.is_empty() {
        return Err((1, "No se pudo determinar la rama. Use --branch".into()));
    }
    let main_branch = if let Ok(out) = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .output()
    {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            if s.contains("origin/main") {
                "main"
            } else {
                "master"
            }
        } else {
            "master"
        }
    } else {
        "master"
    };
    if branch_name == main_branch {
        return Ok("Ya en troncal".into());
    }

    let run = |cmd: &str, cargs: &[&str]| -> bool {
        Command::new(cmd).args(cargs).status().map(|s| s.success()).unwrap_or(false)
    };

    if !run("git", &["checkout", main_branch]) {
        return Err((1, format!("Falló git checkout {}", main_branch)));
    }
    if !run("git", &["pull", "origin", main_branch]) {
        return Err((1, format!("Falló git pull origin {}", main_branch)));
    }
    if !run("git", &["branch", "-d", &branch_name]) {
        let _ = run("git", &["branch", "-D", &branch_name]);
    }
    if delete_remote {
        let _ = run("git", &["push", "origin", "--delete", &branch_name]);
    }
    Ok("Limpieza post-merge completada".into())
}
