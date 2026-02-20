//! Skill iniciar-rama en Rust (contrato skills).
//! Crea rama feat/<slug> o fix/<slug> desde la troncal actualizada.
//! Uso: iniciar_rama.exe <BranchType> <BranchName> [MainBranch] (ej. iniciar_rama.exe feat mi-feature)

use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <feat|fix> <BranchName> [master|main]", args.get(0).unwrap_or(&"iniciar_rama".into()));
        std::process::exit(1);
    }
    let branch_type = args[1].to_lowercase();
    if branch_type != "feat" && branch_type != "fix" {
        eprintln!("BranchType debe ser 'feat' o 'fix'");
        std::process::exit(1);
    }
    let branch_name = args[2]
        .replace(|c: char| c.is_whitespace() || c == '/' || c == '\\', "-")
        .trim()
        .to_string();
    if branch_name.is_empty() {
        eprintln!("BranchName no puede quedar vacío");
        std::process::exit(1);
    }
    let new_branch = format!("{}/{}", branch_type, branch_name);
    let main_branch = args.get(3).map(String::as_str).unwrap_or_else(|| {
        if let Ok(out) = Command::new("git").args(["symbolic-ref", "refs/remotes/origin/HEAD"]).output() {
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
        let mut c = Command::new(cmd);
        c.args(cargs);
        match c.status() {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    };

    // Si la rama ya existe: checkout y merge
    if run("git", &["rev-parse", "--verify", &new_branch]) {
        let _ = run("git", &["fetch", "origin", main_branch]);
        if !run("git", &["checkout", &new_branch]) {
            std::process::exit(1);
        }
        if !run("git", &["merge", &format!("origin/{}", main_branch), "--no-edit"]) {
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    // Fetch, checkout troncal, pull, checkout -b
    if !run("git", &["fetch", "origin"]) {
        eprintln!("Falló git fetch origin");
        std::process::exit(1);
    }
    if !run("git", &["checkout", main_branch]) {
        eprintln!("Falló git checkout {}", main_branch);
        std::process::exit(1);
    }
    if !run("git", &["pull", "origin", main_branch]) {
        eprintln!("Falló git pull origin {}", main_branch);
        std::process::exit(1);
    }
    if !run("git", &["checkout", "-b", &new_branch]) {
        eprintln!("Falló git checkout -b {}", new_branch);
        std::process::exit(1);
    }
    std::process::exit(0);
}
