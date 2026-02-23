//! Skill finalizar-git (pre_pr) en Rust (contrato skills).
//! Push de la rama y creación del PR (gh pr create si está disponible).
//! Uso: push_and_create_pr.exe [--persist <ruta>] [--branch <rama>] [--title <titulo>] [--base main|master]

use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut branch_name = String::new();
    let mut persist = String::new();
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
    if base_branch.is_empty() {
        base_branch = if let Ok(out) = Command::new("git").args(["symbolic-ref", "refs/remotes/origin/HEAD"]).output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                if s.contains("origin/main") { "main" } else { "master" }
            } else { "main" }
        } else { "main" }.to_string();
    }
    if branch_name == base_branch {
        println!("Rama actual es la troncal. No se hace push de PR.");
        std::process::exit(0);
    }

    println!("[Push-And-CreatePR] Rama: {} -> base: {}", branch_name, base_branch);

    let run = |cmd: &str, args: &[&str]| -> bool {
        Command::new(cmd).args(args).status().map(|s| s.success()).unwrap_or(false)
    };

    println!("[1/2] Push origin {}...", branch_name);
    if !run("git", &["push", "origin", &branch_name]) {
        eprintln!("Falló git push origin {}", branch_name);
        std::process::exit(1);
    }
    println!("Push OK.");

    let body = if persist.is_empty() {
        format!("Rama: {}", branch_name)
    } else {
        format!("Documentación: ``{}``", persist)
    };
    let pr_title = if title.is_empty() { branch_name.clone() } else { title };

    println!("[2/2] Creando PR con GitHub CLI (gh)...");
    if run("gh", &["pr", "create", "--base", &base_branch, "--head", &branch_name, "--title", &pr_title, "--body", &body]) {
        println!("PR creado correctamente.");
        std::process::exit(0);
    }

    if let Ok(out) = Command::new("git").args(["config", "--get", "remote.origin.url"]).output() {
        if out.status.success() {
            let url = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Some(repo) = url.split("github.com").nth(1) {
                let repo = repo.trim_start_matches(':').trim_start_matches('/').trim_end_matches(".git").trim_end_matches('/');
                let create_url = format!("https://github.com/{}/compare/{}...{}?expand=1", repo, base_branch, branch_name);
                println!("Crear PR manualmente: {}", create_url);
                println!("Título sugerido: {}", pr_title);
                println!("Body: {}", body);
            }
        }
    }
    std::process::exit(0);
}
