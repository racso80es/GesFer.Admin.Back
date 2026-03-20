//! Skill iniciar-rama — TTY o JSON stdin.

use std::process::Command;
use std::{env, time::Instant};

use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "iniciar-rama";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Body {
    #[serde(default)]
    branch_type: Option<String>,
    #[serde(default)]
    branch_name: Option<String>,
    #[serde(default)]
    main_branch: Option<String>,
}

fn main() {
    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: Body = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let start = Instant::now();
        let branch_type = body.branch_type.unwrap_or_default().to_lowercase();
        let branch_name = body.branch_name.unwrap_or_default();
        let main_override = body.main_branch.filter(|s| !s.is_empty());

        match run_logic(&branch_type, &branch_name, main_override.as_deref()) {
            Ok(msg) => {
                let res = CapsuleResponse::skill(
                    SKILL_ID,
                    true,
                    0,
                    &msg,
                    vec![FeedbackEntry::info("git", &msg)],
                    serde_json::json!({ "branch": format!("{}/{}", branch_type, sanitize(&branch_name)) }),
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

    // Legacy argv: <feat|fix> <BranchName> [main]
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Uso: {} <feat|fix> <BranchName> [master|main]",
            args.get(0).unwrap_or(&"iniciar_rama".into())
        );
        std::process::exit(1);
    }
    let branch_type = args[1].to_lowercase();
    let branch_name = args[2].clone();
    let main_override = args.get(3).map(|s| s.as_str());
    match run_logic(&branch_type, &branch_name, main_override) {
        Ok(_) => std::process::exit(0),
        Err((code, _)) => std::process::exit(code),
    }
}

fn sanitize(name: &str) -> String {
    name.replace(|c: char| c.is_whitespace() || c == '/' || c == '\\', "-")
        .trim()
        .to_string()
}

fn run_logic(branch_type: &str, branch_name: &str, main_override: Option<&str>) -> Result<String, (i32, String)> {
    if branch_type != "feat" && branch_type != "fix" {
        return Err((1, "BranchType debe ser 'feat' o 'fix'".into()));
    }
    let branch_name = sanitize(branch_name);
    if branch_name.is_empty() {
        return Err((1, "BranchName no puede quedar vacío".into()));
    }
    let new_branch = format!("{}/{}", branch_type, branch_name);
    let main_branch = main_override.unwrap_or_else(|| {
        if let Ok(out) = Command::new("git")
            .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                if s.contains("origin/main") {
                    return "main";
                }
                if s.contains("origin/master") {
                    return "master";
                }
            }
        }
        "master"
    });

    let run = |cmd: &str, cargs: &[&str]| -> bool {
        Command::new(cmd).args(cargs).status().map(|s| s.success()).unwrap_or(false)
    };

    if run("git", &["rev-parse", "--verify", &new_branch]) {
        let _ = run("git", &["fetch", "origin", main_branch]);
        if !run("git", &["checkout", &new_branch]) {
            return Err((1, "Falló git checkout".into()));
        }
        if !run("git", &["merge", &format!("origin/{}", main_branch), "--no-edit"]) {
            return Err((1, "Falló git merge".into()));
        }
        return Ok(format!("Rama existente actualizada: {}", new_branch));
    }

    if !run("git", &["fetch", "origin"]) {
        return Err((1, "Falló git fetch origin".into()));
    }
    if !run("git", &["checkout", main_branch]) {
        return Err((1, format!("Falló git checkout {}", main_branch)));
    }
    if !run("git", &["pull", "origin", main_branch]) {
        return Err((1, format!("Falló git pull origin {}", main_branch)));
    }
    if !run("git", &["checkout", "-b", &new_branch]) {
        return Err((1, format!("Falló git checkout -b {}", new_branch)));
    }
    Ok(format!("Rama creada: {}", new_branch))
}
