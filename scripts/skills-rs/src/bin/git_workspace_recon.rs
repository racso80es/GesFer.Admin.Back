//! Skill git-workspace-recon — inspección de workspace Git (status -s + diff --stat).

use std::path::PathBuf;
use std::process::Command;

use chrono::Utc;
use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "git-workspace-recon";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ReconRequest {
    #[serde(default)]
    target_path: Option<String>,
}

#[derive(Debug, Parser)]
#[command(name = "git_workspace_recon", version, about = "Recon de workspace Git: status -s + diff --stat")]
struct CliArgs {
    /// Ruta opcional donde ejecutar Git (cwd).
    #[arg(long = "target-path")]
    target_path: Option<String>,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: ReconRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let (res, exit_code) = run_recon(body.target_path, start);
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    // Modo CLI (humano / runner): flags
    let args = CliArgs::parse();
    let (res, exit_code) = run_recon(args.target_path, start);
    let _ = write_capsule_response(&res);
    std::process::exit(exit_code);
}

fn run_recon(target_path: Option<String>, start: std::time::Instant) -> (CapsuleResponse, i32) {
    let mut feedback: Vec<FeedbackEntry> = vec![];

    let cwd = target_path
        .as_ref()
        .map(|p| p.trim().trim_matches('"').to_string())
        .filter(|p| !p.is_empty());

    let cwd_path: Option<PathBuf> = cwd.as_ref().map(PathBuf::from);

    let status_start = std::time::Instant::now();
    let status_out = run_git(&["status", "-s"], cwd_path.as_deref());
    feedback.push(FeedbackEntry {
        phase: "git".to_string(),
        level: if status_out.exit_code == 0 {
            gesfer_skills::FeedbackLevel::Info
        } else {
            gesfer_skills::FeedbackLevel::Error
        },
        message: "git status -s".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        detail: if status_out.exit_code == 0 {
            None
        } else {
            Some(status_out.combined.clone())
        },
        duration_ms: Some(status_start.elapsed().as_millis() as u64),
    });
    if status_out.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            status_out.exit_code,
            "Git status falló",
            feedback,
            serde_json::json!({
                "targetPath": cwd.unwrap_or_else(|| ".".to_string()),
                "status": { "raw": status_out.combined, "entries": [] },
                "diffStat": { "raw": "", "files": [] }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, status_out.exit_code);
    }

    let diff_start = std::time::Instant::now();
    let diff_out = run_git(&["diff", "--stat"], cwd_path.as_deref());
    feedback.push(FeedbackEntry {
        phase: "git".to_string(),
        level: if diff_out.exit_code == 0 {
            gesfer_skills::FeedbackLevel::Info
        } else {
            gesfer_skills::FeedbackLevel::Error
        },
        message: "git diff --stat".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        detail: if diff_out.exit_code == 0 {
            None
        } else {
            Some(diff_out.combined.clone())
        },
        duration_ms: Some(diff_start.elapsed().as_millis() as u64),
    });
    if diff_out.exit_code != 0 {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            diff_out.exit_code,
            "Git diff --stat falló",
            feedback,
            serde_json::json!({
                "targetPath": cwd.unwrap_or_else(|| ".".to_string()),
                "status": {
                    "raw": status_out.combined,
                    "entries": parse_status_entries(&status_out.combined)
                },
                "diffStat": { "raw": diff_out.combined, "files": [] }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, diff_out.exit_code);
    }

    let entries = parse_status_entries(&status_out.combined);
    let files = parse_diff_stat_files(&diff_out.combined);

    let res = CapsuleResponse::skill(
        SKILL_ID,
        true,
        0,
        if entries.is_empty() && files.is_empty() {
            "Workspace limpio"
        } else {
            "Recon completado"
        },
        feedback,
        serde_json::json!({
            "targetPath": cwd.unwrap_or_else(|| ".".to_string()),
            "status": { "raw": status_out.combined, "entries": entries },
            "diffStat": { "raw": diff_out.combined, "files": files }
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, 0)
}

struct CmdOut {
    exit_code: i32,
    combined: String,
}

fn run_git(args: &[&str], cwd: Option<&std::path::Path>) -> CmdOut {
    let mut cmd = Command::new("git");
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    match cmd.output() {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let code = o.status.code().unwrap_or(-1);
            let combined = format!("{}\n{}", out, err).trim().to_string();
            CmdOut {
                exit_code: code,
                combined,
            }
        }
        Err(e) => CmdOut {
            exit_code: 1,
            combined: e.to_string(),
        },
    }
}

fn parse_status_entries(raw: &str) -> Vec<serde_json::Value> {
    // Formato: XY <path> (o "R  old -> new"). También puede ser "?? <path>".
    let mut entries = vec![];
    for line in raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.len() < 3 {
            continue;
        }
        let code = line.chars().take(2).collect::<String>();
        let rest = line.chars().skip(3).collect::<String>().trim().to_string();
        if rest.is_empty() {
            continue;
        }
        if rest.contains(" -> ") {
            let parts: Vec<&str> = rest.split(" -> ").collect();
            if parts.len() == 2 {
                entries.push(serde_json::json!({
                    "code": code.trim(),
                    "path": parts[0].trim(),
                    "path2": parts[1].trim(),
                    "kind": kind_from_status_code(&code)
                }));
                continue;
            }
        }
        entries.push(serde_json::json!({
            "code": code.trim(),
            "path": rest,
            "kind": kind_from_status_code(&code)
        }));
    }
    entries
}

fn kind_from_status_code(code: &str) -> &'static str {
    let c = code.trim();
    if c.contains('?') {
        return "untracked";
    }
    if c.contains('A') {
        return "added";
    }
    if c.contains('D') {
        return "deleted";
    }
    if c.contains('R') {
        return "renamed";
    }
    if c.contains('M') {
        return "modified";
    }
    "other"
}

fn parse_diff_stat_files(raw: &str) -> Vec<serde_json::Value> {
    // Formato típico por línea:
    // path | 3 ++-
    // La última línea suele ser: "N files changed, X insertions(+), Y deletions(-)"
    let mut files = vec![];
    for line in raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.contains("files changed") {
            continue;
        }
        let Some((path_part, stat_part)) = line.split_once('|') else {
            continue;
        };
        let path = path_part.trim();
        let stat = stat_part.trim();
        if path.is_empty() || stat.is_empty() {
            continue;
        }
        // stat: "<n> <symbols>"
        let mut it = stat.split_whitespace();
        let changes: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let symbols = it.next().unwrap_or("");
        let insertions = symbols.chars().filter(|c| *c == '+').count() as i32;
        let deletions = symbols.chars().filter(|c| *c == '-').count() as i32;
        files.push(serde_json::json!({
            "path": path,
            "changes": changes,
            "insertions": insertions,
            "deletions": deletions
        }));
    }
    files
}

