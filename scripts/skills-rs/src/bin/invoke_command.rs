//! Skill invoke-command — CLI legacy (TTY) o JSON stdin (agente). Ver SddIA/norms/capsule-json-io.md.

use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

use chrono::Utc;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "invoke-command";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct InvokeRequest {
    #[serde(default)]
    command: Option<String>,
    #[serde(default)]
    command_file: Option<String>,
    #[serde(default)]
    fase: Option<String>,
    #[serde(default)]
    contexto: Option<String>,
}

fn main() {
    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: InvokeRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let command = body
            .command
            .filter(|s| !s.is_empty())
            .or_else(|| {
                body.command_file.as_ref().and_then(|p| {
                    fs::read_to_string(p.trim_matches('"').trim())
                        .ok()
                        .map(|s| s.trim().to_string())
                })
            })
            .unwrap_or_default();
        let fase = body.fase.unwrap_or_else(|| "Accion".to_string());
        let contexto = body.contexto.unwrap_or_else(|| "GesFer".to_string());

        if command.is_empty() {
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                1,
                "request.command o request.commandFile obligatorio",
                vec![FeedbackEntry::error("validate", "Sin comando", None)],
                serde_json::json!({}),
                None,
            );
            let _ = write_capsule_response(&res);
            std::process::exit(1);
        }

        let start = std::time::Instant::now();
        let (exit_code, out_str, ok) = run_shell(&command);
        append_log_line(&fase, &contexto, &command, ok, exit_code, &out_str);

        let feedback = vec![
            FeedbackEntry::info("exec", if ok { "Comando finalizado" } else { "Comando falló" }),
        ];
        let res = CapsuleResponse::skill(
            SKILL_ID,
            ok,
            exit_code,
            if ok {
                "Comando ejecutado"
            } else {
                "Comando falló"
            },
            feedback,
            serde_json::json!({
                "outputPreview": out_str.chars().take(2000).collect::<String>(),
                "exitCode": exit_code
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    // Modo TTY / legacy: argumentos
    let args: Vec<String> = env::args().collect();
    let mut command = String::new();
    let mut fase = "Accion".to_string();
    let mut contexto = "GesFer".to_string();
    let mut i = 1;
    while i < args.len() {
        if (args[i] == "--command" || args[i] == "-Command") && i + 1 < args.len() {
            command = args[i + 1].clone();
            i += 2;
            continue;
        }
        if (args[i] == "--command-file" || args[i] == "-CommandFile") && i + 1 < args.len() {
            let path = args[i + 1].trim_matches('"').trim();
            if let Ok(s) = fs::read_to_string(path) {
                command = s.trim().to_string();
            }
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
    if command.is_empty() {
        eprintln!(
            "Uso: {} --command \"<comando>\" | --command-file <ruta> [--fase Accion] [--contexto GesFer]",
            args.get(0).unwrap_or(&"invoke_command".into())
        );
        std::process::exit(1);
    }

    let (exit_code, out_str, ok) = run_shell(&command);
    append_log_line(&fase, &contexto, &command, ok, exit_code, &out_str);

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

fn run_shell(command: &str) -> (i32, String, bool) {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-NoProfile", "-Command", command])
            .output()
    } else {
        Command::new("sh").args(["-c", command]).output()
    };

    match output {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let code = o.status.code().unwrap_or(-1);
            let text = format!("{}\n{}", out, err).trim().to_string();
            (code, text, o.status.success())
        }
        Err(e) => (1, e.to_string(), false),
    }
}

fn append_log_line(fase: &str, contexto: &str, command: &str, ok: bool, exit_code: i32, out_str: &str) {
    let branch = Command::new("git")
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
        .unwrap_or_else(|| "unknown".to_string());

    let log_dir = format!("docs/diagnostics/{}", branch);
    let _ = fs::create_dir_all(&log_dir);
    let log_path = format!("{}/execution_history.json", log_dir);
    let timestamp = Utc::now().to_rfc3339();
    let status_str = if ok { "Success" } else { "Failed" };
    let log_entry = format!(
        "{{\"Timestamp\":\"{}\",\"Fase\":\"{}\",\"Contexto\":\"{}\",\"Command\":\"{}\",\"Status\":\"{}\",\"ExitCode\":{},\"Output\":\"{}\"}}\n",
        timestamp,
        fase.replace('"', "\\\""),
        contexto.replace('"', "\\\""),
        command.replace('"', "\\\"").replace('\n', " "),
        status_str,
        exit_code,
        out_str
            .replace('"', "\\\"")
            .replace('\n', " ")
            .chars()
            .take(500)
            .collect::<String>()
    );
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = f.write_all(log_entry.as_bytes());
    }
}
