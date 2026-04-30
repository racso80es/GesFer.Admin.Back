//! Skill invoke-commit — git add + commit — TTY o JSON stdin.

use chrono::Utc;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::{env, time::Instant};

use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "invoke-commit";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Body {
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    files: Option<String>,
    #[serde(default)]
    all: Option<bool>,
    #[serde(rename = "type", default)]
    commit_type: Option<String>,
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    fase: Option<String>,
    #[serde(default)]
    contexto: Option<String>,
}

fn main() {
    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: Body = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let start = Instant::now();
        let message = body.message.unwrap_or_default();
        let files = body.files.unwrap_or_default();
        let all = body.all.unwrap_or(false);
        let commit_type = body.commit_type.unwrap_or_else(|| "feat".into());
        let scope = body.scope.unwrap_or_default();
        let fase = body.fase.unwrap_or_else(|| "Accion".into());
        let contexto = body.contexto.unwrap_or_else(|| "GesFer".into());

        if message.is_empty() {
            emit_err(
                &fase,
                &contexto,
                "Falta message",
                1,
                start,
                vec![FeedbackEntry::error("validate", "Falta message", None)],
            );
        }
        if files.is_empty() && !all {
            emit_err(
                &fase,
                &contexto,
                "Falta files o all",
                1,
                start,
                vec![FeedbackEntry::error("validate", "Falta files o all", None)],
            );
        }
        if !files.is_empty() && all {
            emit_err(
                &fase,
                &contexto,
                "files y all exclusivos",
                1,
                start,
                vec![FeedbackEntry::error("validate", "files y all exclusivos", None)],
            );
        }

        match do_commit(
            &message,
            &files,
            all,
            &commit_type,
            &scope,
            &fase,
            &contexto,
        ) {
            Ok(out) => {
                let res = CapsuleResponse::skill(
                    SKILL_ID,
                    true,
                    0,
                    "Commit OK",
                    vec![FeedbackEntry::info("git", "Commit creado")],
                    serde_json::json!({ "output": out }),
                    Some(start.elapsed().as_millis() as u64),
                );
                let _ = write_capsule_response(&res);
                std::process::exit(0);
            }
            Err((code, msg, fb)) => {
                let res = CapsuleResponse::skill(
                    SKILL_ID,
                    false,
                    code,
                    &msg,
                    fb,
                    serde_json::json!({}),
                    Some(start.elapsed().as_millis() as u64),
                );
                let _ = write_capsule_response(&res);
                std::process::exit(code);
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    let mut message = String::new();
    let mut files = String::new();
    let mut all = false;
    let mut commit_type = "feat".to_string();
    let mut scope = String::new();
    let mut fase = "Accion".to_string();
    let mut contexto = "GesFer".to_string();
    let mut i = 1;
    while i < args.len() {
        if (args[i] == "--message" || args[i] == "-m") && i + 1 < args.len() {
            message = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--files" || args[i] == "-Files") && i + 1 < args.len() {
            files = args[i + 1].clone();
            i += 2;
            continue;
        }
        if args[i] == "--all" || args[i] == "-a" {
            all = true;
            i += 1;
            continue;
        }
        if (args[i] == "--type" || args[i] == "-Type") && i + 1 < args.len() {
            commit_type = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--scope" || args[i] == "-Scope") && i + 1 < args.len() {
            scope = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--fase" || args[i] == "-Fase") && i + 1 < args.len() {
            fase = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--contexto" || args[i] == "-Contexto") && i + 1 < args.len() {
            contexto = args[i + 1].clone();
            i += 2;
            continue;
        }
        i += 1;
    }
    if message.is_empty() {
        eprintln!("Uso: invoke_commit --message \"msg\" [--files \"a,b\"] [--all] [--type feat] [--scope x]");
        std::process::exit(1);
    }
    if files.is_empty() && !all {
        eprintln!("Falta --files o --all");
        std::process::exit(1);
    }
    if !files.is_empty() && all {
        eprintln!("--files y --all no juntos");
        std::process::exit(1);
    }

    match do_commit(
        &message,
        &files,
        all,
        &commit_type,
        &scope,
        &fase,
        &contexto,
    ) {
        Ok(_) => std::process::exit(0),
        Err((code, _, _)) => std::process::exit(code),
    }
}

fn emit_err(
    fase: &str,
    contexto: &str,
    msg: &str,
    code: i32,
    start: Instant,
    fb: Vec<FeedbackEntry>,
) -> ! {
    let _ = (fase, contexto);
    let res = CapsuleResponse::skill(
        SKILL_ID,
        false,
        code,
        msg,
        fb,
        serde_json::json!({}),
        Some(start.elapsed().as_millis() as u64),
    );
    let _ = write_capsule_response(&res);
    std::process::exit(code);
}

fn do_commit(
    message: &str,
    files: &str,
    all: bool,
    commit_type: &str,
    scope: &str,
    fase: &str,
    contexto: &str,
) -> Result<String, (i32, String, Vec<FeedbackEntry>)> {
    let branch = git_branch();
    let full_message = if scope.is_empty() {
        format!("{}: {}", commit_type, message.trim())
    } else {
        format!("{}({}): {}", commit_type, scope.trim(), message.trim())
    };

    let run = |cargs: &[&str]| -> (bool, String) {
        match Command::new("git").args(cargs).output() {
            Ok(o) => {
                let combined = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&o.stdout),
                    String::from_utf8_lossy(&o.stderr)
                )
                .trim()
                .to_string();
                (o.status.success(), combined)
            }
            Err(e) => (false, e.to_string()),
        }
    };

    if all {
        let (ok, out) = run(&["add", "-A"]);
        if !ok {
            log_line(fase, contexto, "git add -A", false, 1, &out, &branch);
            return Err((
                1,
                format!("git add -A: {}", out),
                vec![FeedbackEntry::error("git", "git add falló", Some(&out))],
            ));
        }
    } else {
        let file_list: Vec<&str> = files
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if file_list.is_empty() {
            return Err((
                1,
                "--files vacío".into(),
                vec![FeedbackEntry::error("validate", "files vacío", None)],
            ));
        }
        let mut add_args: Vec<&str> = vec!["add"];
        add_args.extend(file_list.iter().copied());
        let (ok, out) = run(&add_args);
        if !ok {
            log_line(
                fase,
                contexto,
                &format!("git add {}", files),
                false,
                1,
                &out,
                &branch,
            );
            return Err((
                1,
                format!("git add: {}", out),
                vec![FeedbackEntry::error("git", "git add falló", Some(&out))],
            ));
        }
    }

    let (ok, out) = run(&["commit", "-m", &full_message]);
    if !ok {
        log_line(
            fase,
            contexto,
            &format!("git commit -m \"{}\"", full_message),
            false,
            1,
            &out,
            &branch,
        );
        return Err((
            1,
            format!("git commit: {}", out),
            vec![FeedbackEntry::error("git", "commit falló", Some(&out))],
        ));
    }

    let cmd_log = if all {
        format!("git add -A; git commit -m \"{}\"", full_message)
    } else {
        format!("git add {}; git commit -m \"{}\"", files, full_message)
    };
    log_line(fase, contexto, &cmd_log, true, 0, &out, &branch);
    Ok(out)
}

fn git_branch() -> String {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn log_line(
    fase: &str,
    contexto: &str,
    command: &str,
    ok: bool,
    code: i32,
    output: &str,
    branch: &str,
) {
    let log_dir = format!("docs/diagnostics/{}", branch);
    let _ = fs::create_dir_all(&log_dir);
    let log_path = format!("{}/execution_history.json", log_dir);
    let status = if ok { "Success" } else { "Failed" };
    let line = format!(
        "{{\"Timestamp\":\"{}\",\"Fase\":\"{}\",\"Contexto\":\"{}\",\"Command\":\"{}\",\"Status\":\"{}\",\"ExitCode\":{},\"Output\":\"{}\"}}\n",
        Utc::now().to_rfc3339(),
        fase.replace('"', "\\\""),
        contexto.replace('"', "\\\""),
        command.replace('"', "\\\"").replace('\n', " "),
        status,
        code,
        output
            .replace('"', "\\\"")
            .replace('\n', " ")
            .chars()
            .take(500)
            .collect::<String>()
    );
    if let Ok(mut f) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = f.write_all(line.as_bytes());
    }
}
