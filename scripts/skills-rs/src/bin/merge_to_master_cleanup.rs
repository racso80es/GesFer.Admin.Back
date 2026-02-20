//! Skill finalizar-git (post_pr) en Rust (contrato skills).
//! Post-merge: checkout master, pull, eliminar rama local y opcionalmente remota.
//! Uso: merge_to_master_cleanup.exe [--branch <BranchName>] [--delete-remote]

use std::process::Command;
use std::env;

fn main() {
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
    if branch_name.is_empty() {
        if let Ok(out) = Command::new("git").args(["branch", "--show-current"]).output() {
            if out.status.success() {
                branch_name = String::from_utf8_lossy(&out.stdout).trim().to_string();
            }
        }
    }
    if branch_name.is_empty() {
        eprintln!("No se pudo determinar la rama. Use --branch <nombre>");
        std::process::exit(1);
    }
    let main_branch = if let Ok(out) = Command::new("git").args(["symbolic-ref", "refs/remotes/origin/HEAD"]).output() {
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
        std::process::exit(0);
    }

    let run = |cmd: &str, cargs: &[&str]| -> bool {
        Command::new(cmd).args(cargs).status().map(|s| s.success()).unwrap_or(false)
    };

    if !run("git", &["checkout", main_branch]) {
        eprintln!("Falló git checkout {}", main_branch);
        std::process::exit(1);
    }
    if !run("git", &["pull", "origin", main_branch]) {
        eprintln!("Falló git pull origin {}", main_branch);
        std::process::exit(1);
    }
    if !run("git", &["branch", "-d", &branch_name]) {
        let _ = run("git", &["branch", "-D", &branch_name]);
    }
    if delete_remote {
        let _ = run("git", &["push", "origin", "--delete", &branch_name]);
    }
    std::process::exit(0);
}
