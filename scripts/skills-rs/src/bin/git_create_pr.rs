//! Skill git-create-pr — crea Pull Request con gh y devuelve URL.

use std::process::Command;

use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "git-create-pr";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PrRequest {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    base_branch: Option<String>,
}

#[derive(Debug, Parser)]
#[command(name = "git_create_pr", version, about = "Crear PR vía gh pr create")]
struct CliArgs {
    #[arg(long)]
    title: String,
    #[arg(long)]
    body: String,
    #[arg(long = "base-branch", default_value = "main")]
    base_branch: String,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: PrRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let title = body.title.unwrap_or_default();
        let pr_body = body.body.unwrap_or_default();
        let base = body.base_branch.unwrap_or_else(|| "main".to_string());

        if title.trim().is_empty() || pr_body.trim().is_empty() {
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                1,
                "request.title y request.body son obligatorios",
                vec![FeedbackEntry::error("validate", "Faltan campos obligatorios", None)],
                serde_json::json!({}),
                Some(start.elapsed().as_millis() as u64),
            );
            let _ = write_capsule_response(&res);
            std::process::exit(1);
        }

        let (res, exit_code) = run(title, pr_body, base, start);
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    let args = CliArgs::parse();
    let (res, exit_code) = run(args.title, args.body, args.base_branch, start);
    let _ = write_capsule_response(&res);
    std::process::exit(exit_code);
}

fn run(title: String, body: String, base_branch: String, start: std::time::Instant) -> (CapsuleResponse, i32) {
    let mut feedback: Vec<FeedbackEntry> = vec![FeedbackEntry::info("gh", "gh pr create")];

    let create = run_gh(&[
        "pr",
        "create",
        "--title",
        title.trim(),
        "--body",
        body.as_str(),
        "--base",
        base_branch.trim(),
    ]);

    if create.exit_code == 0 {
        let url = extract_pr_url(&create.combined).unwrap_or_default();
        let res = CapsuleResponse::skill(
            SKILL_ID,
            true,
            0,
            "PR creado",
            feedback,
            serde_json::json!({
                "created": true,
                "prUrl": url,
                "create": { "exitCode": create.exit_code, "output": create.combined }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, 0);
    }

    let lower = create.combined.to_lowercase();
    let already_exists = lower.contains("a pull request already exists")
        || lower.contains("pull request already exists")
        || lower.contains("already exists");

    if already_exists {
        feedback.push(FeedbackEntry::warning("gh", "PR ya existe; resolviendo URL", None));
        let view = run_gh(&["pr", "view", "--json", "url", "--jq", ".url"]);
        if view.exit_code == 0 {
            let url = view.combined.lines().next().unwrap_or("").trim().to_string();
            let res = CapsuleResponse::skill(
                SKILL_ID,
                true,
                0,
                "PR ya existía",
                feedback,
                serde_json::json!({
                    "created": false,
                    "prUrl": url,
                    "create": { "exitCode": create.exit_code, "output": create.combined },
                    "view": { "exitCode": view.exit_code, "output": view.combined }
                }),
                Some(start.elapsed().as_millis() as u64),
            );
            return (res, 0);
        }
    }

    feedback.push(FeedbackEntry::error("gh", "gh pr create falló", Some(&create.combined)));
    let res = CapsuleResponse::skill(
        SKILL_ID,
        false,
        create.exit_code,
        "No se pudo crear el PR",
        feedback,
        serde_json::json!({
            "created": false,
            "prUrl": null,
            "create": { "exitCode": create.exit_code, "output": create.combined }
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, create.exit_code)
}

struct CmdOut {
    exit_code: i32,
    combined: String,
}

fn run_gh(args: &[&str]) -> CmdOut {
    let output = Command::new("gh").args(args).output();
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

fn extract_pr_url(text: &str) -> Option<String> {
    // gh suele imprimir la URL en stdout en una línea.
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with("http://") || l.starts_with("https://") {
            return Some(l.to_string());
        }
    }
    None
}

