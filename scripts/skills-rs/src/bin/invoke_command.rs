//! Skill invoke-command en Rust (contrato skills).
//! Ejecuta un comando de sistema y registra en docs/diagnostics/<branch>/execution_history.json.
//! Uso: invoke_command.exe --command "<comando>" | --command-file <ruta> [--fase Accion] [--contexto GesFer]
//! Acepta -Command/-Fase (PowerShell) y --command-file para leer comando desde archivo (evita inyección en terminal).

use std::process::Command;
use std::env;
use std::fs;
use std::io::Write;
use chrono::Utc;

fn main() {
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
        eprintln!("Uso: {} --command \"<comando>\" | --command-file <ruta> [--fase Accion] [--contexto GesFer]", args.get(0).unwrap_or(&"invoke_command".into()));
        std::process::exit(1);
    }

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
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-NoProfile", "-Command", &command])
            .output()
    } else {
        Command::new("sh").args(["-c", &command]).output()
    };

    let (status_str, exit_code, out_str) = match output {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let code = o.status.code().unwrap_or(-1);
            (
                if o.status.success() { "Success" } else { "Failed" }.to_string(),
                code,
                format!("{}\n{}", out, err).trim().to_string(),
            )
        }
        Err(e) => ("Failed".to_string(), 1, e.to_string()),
    };

    let log_entry = format!(
        "{{\"Timestamp\":\"{}\",\"Fase\":\"{}\",\"Contexto\":\"{}\",\"Command\":\"{}\",\"Status\":\"{}\",\"ExitCode\":{},\"Output\":\"{}\"}}\n",
        timestamp,
        fase.replace('"', "\\\""),
        contexto.replace('"', "\\\""),
        command.replace('"', "\\\"").replace('\n', " "),
        status_str,
        exit_code,
        out_str.replace('"', "\\\"").replace('\n', " ").chars().take(500).collect::<String>()
    );
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = f.write_all(log_entry.as_bytes());
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
