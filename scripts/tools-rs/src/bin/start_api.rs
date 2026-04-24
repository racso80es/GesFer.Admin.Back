use std::{fs, path::PathBuf, process::Command};

use clap::Parser;
use gesfer_tools::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const TOOL_ID: &str = "start-api";

#[derive(Debug, Parser)]
#[command(name = "start_api")]
struct Cli {
    #[arg(long)]
    working_dir: Option<String>,
    #[arg(long)]
    command: Option<String>,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    output_json: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct JsonRequest {
    #[serde(default)]
    working_dir: Option<String>,
    #[serde(default)]
    command: Option<String>,
    #[serde(default)]
    output_path: Option<String>,
    #[serde(default)]
    output_json: Option<bool>,
}

fn repo_root() -> PathBuf {
    std::env::var("GESFER_REPO_ROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn resolve_path(root: &PathBuf, p: &str) -> PathBuf {
    let p = PathBuf::from(p);
    if p.is_absolute() { p } else { root.join(p) }
}

fn main() {
    let started = std::time::Instant::now();
    let mut feedback = vec![FeedbackEntry::info("init", "Iniciando start-api (Rust)")];

    let capsule_req = try_read_capsule_request().ok().flatten();
    let mut cli = Cli::parse();
    if let Some(req) = capsule_req {
        if let Ok(j) = serde_json::from_value::<JsonRequest>(req.request) {
            if j.working_dir.is_some() { cli.working_dir = j.working_dir; }
            if j.command.is_some() { cli.command = j.command; }
            if j.output_path.is_some() { cli.output_path = j.output_path; }
            if let Some(v) = j.output_json { cli.output_json = v; }
        }
    }

    let root = repo_root();
    let wd = cli
        .working_dir
        .as_deref()
        .map(|p| resolve_path(&root, p))
        .unwrap_or_else(|| root.join("src/GesFer.Admin.Back.Api"));
    let cmdline = cli.command.clone().unwrap_or_else(|| "dotnet run".to_string());

    feedback.push(FeedbackEntry::info("api", &format!("Lanzando API: {} (wd: {})", cmdline, wd.display())));

    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(&cmdline).current_dir(&wd);

    let res = match cmd.spawn() {
        Ok(_) => CapsuleResponse::tool(
            TOOL_ID,
            true,
            0,
            "API lanzada",
            feedback,
            serde_json::json!({ "workingDir": wd.display().to_string(), "command": cmdline }),
            Some(started.elapsed().as_millis() as u64),
        ),
        Err(e) => {
            let mut fb = feedback;
            fb.push(FeedbackEntry::error("api", "No se pudo iniciar la API", Some(&e.to_string())));
            CapsuleResponse::tool(
                TOOL_ID,
                false,
                1,
                "No se pudo iniciar la API",
                fb,
                serde_json::json!({ "workingDir": wd.display().to_string(), "command": cmdline }),
                Some(started.elapsed().as_millis() as u64),
            )
        }
    };

    if let Some(p) = cli.output_path {
        let _ = fs::write(p, serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string()));
    }
    if cli.output_json {
        let _ = write_capsule_response(&res);
    }
    std::process::exit(res.exit_code);
}

