//! Skill finalizar-git (pre_pr) — push + PR — TTY o JSON stdin.

use std::fs;
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
    persist: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    body_file: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    base: Option<String>,
}

fn main() {
    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: Body = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let start = Instant::now();
        match run_push_pr(
            body.branch_name.as_deref(),
            body.persist.as_deref().unwrap_or(""),
            body.body.as_deref().unwrap_or(""),
            body.body_file.as_deref().unwrap_or(""),
            body.title.as_deref().unwrap_or(""),
            body.base.as_deref(),
        ) {
            Ok(msg) => {
                let res = CapsuleResponse::skill(
                    SKILL_ID,
                    true,
                    0,
                    &msg,
                    vec![FeedbackEntry::info("pr", &msg)],
                    serde_json::json!({ "message": msg }),
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
                    vec![FeedbackEntry::error("pr", &msg, None)],
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
    let mut persist = String::new();
    let mut body = String::new();
    let mut body_file = String::new();
    let mut title = String::new();
    let mut base_branch = String::new();
    let mut i = 1;
    while i < args.len() {
        if (args[i] == "--branch" || args[i] == "-BranchName") && i + 1 < args.len() {
            branch_name = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--persist" || args[i] == "-Persist") && i + 1 < args.len() {
            persist = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--body" || args[i] == "-Body") && i + 1 < args.len() {
            body = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--body-file" || args[i] == "-BodyFile") && i + 1 < args.len() {
            body_file = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--title" || args[i] == "-Title") && i + 1 < args.len() {
            title = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--base" || args[i] == "-BaseBranch") && i + 1 < args.len() {
            base_branch = args[i + 1].clone();
            i += 2;
            continue;
        }
        i += 1;
    }

    let branch_opt = if branch_name.is_empty() {
        None
    } else {
        Some(branch_name.as_str())
    };
    let base_opt = if base_branch.is_empty() {
        None
    } else {
        Some(base_branch.as_str())
    };
    match run_push_pr(
        branch_opt,
        &persist,
        &body,
        &body_file,
        &title,
        base_opt,
    ) {
        Ok(_) => std::process::exit(0),
        Err((code, _)) => std::process::exit(code),
    }
}

fn run_push_pr(
    branch_name_in: Option<&str>,
    persist: &str,
    body: &str,
    body_file: &str,
    title: &str,
    base_in: Option<&str>,
) -> Result<String, (i32, String)> {
    let run = |cmd: &str, args: &[&str]| -> bool {
        Command::new(cmd).args(args).status().map(|s| s.success()).unwrap_or(false)
    };

    let mut branch_name = branch_name_in.map(|s| s.to_string()).unwrap_or_default();
    if branch_name.is_empty() {
        if let Ok(out) = Command::new("git").args(["branch", "--show-current"]).output() {
            if out.status.success() {
                branch_name = String::from_utf8_lossy(&out.stdout).trim().to_string();
            }
        }
    }
    if branch_name.is_empty() {
        return Err((1, "No se pudo determinar la rama".into()));
    }

    let mut base_branch = base_in.unwrap_or("").to_string();
    if base_branch.is_empty() {
        base_branch = if let Ok(out) = Command::new("git")
            .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                if s.contains("origin/main") {
                    "main".into()
                } else {
                    "master".into()
                }
            } else {
                "main".into()
            }
        } else {
            "main".into()
        };
    }

    if branch_name == base_branch {
        return Ok("Rama actual es la troncal. No se hace push de PR.".into());
    }

    if !run("git", &["push", "origin", &branch_name]) {
        return Err((1, format!("Falló git push origin {}", branch_name)));
    }

    let pr_body = if !body_file.is_empty() {
        let from_file = fs::read_to_string(body_file.trim_matches('"').trim())
            .unwrap_or_default()
            .trim()
            .to_string();
        if from_file.is_empty() && !persist.is_empty() {
            format!("Documentación: ``{}``", persist)
        } else if from_file.is_empty() {
            format!("Rama: {}", branch_name)
        } else {
            from_file
        }
    } else if !body.is_empty() {
        body.to_string()
    } else if !persist.is_empty() {
        format!("Documentación: ``{}``", persist)
    } else {
        format!("Rama: {}", branch_name)
    };
    let pr_title = if title.is_empty() {
        branch_name.clone()
    } else {
        title.to_string()
    };

    if run(
        "gh",
        &[
            "pr",
            "create",
            "--base",
            &base_branch,
            "--head",
            &branch_name,
            "--title",
            &pr_title,
            "--body",
            &pr_body,
        ],
    ) {
        return Ok("PR creado correctamente".into());
    }

    if let Ok(out) = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
    {
        if out.status.success() {
            let url = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Some(repo) = url.split("github.com").nth(1) {
                let repo = repo
                    .trim_start_matches(':')
                    .trim_start_matches('/')
                    .trim_end_matches(".git")
                    .trim_end_matches('/');
                let create_url = format!(
                    "https://github.com/{}/compare/{}...{}?expand=1",
                    repo, base_branch, branch_name
                );
                return Ok(format!(
                    "gh no disponible. Crear PR manualmente: {} (título: {})",
                    create_url, pr_title
                ));
            }
        }
    }
    Ok("Push OK; no se pudo construir URL de PR".into())
}
