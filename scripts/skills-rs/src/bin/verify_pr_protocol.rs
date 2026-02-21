//! Skill verify-pr-protocol
//! Enforces the PR Acceptance Protocol:
//! 1. Nomenclature (validate-nomenclatura.ps1)
//! 2. Compilation (dotnet build)
//! 3. Tests (dotnet test)

use std::process::{Command, exit};

fn main() {
    println!("[VERIFY-PR-PROTOCOL] Starting PR Acceptance Protocol...");

    // 1. Nomenclature Check
    println!("[VERIFY-PR-PROTOCOL] 1/3 Checking Nomenclature...");
    // Attempt to find a suitable shell for the PS1 script
    let shell = if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "pwsh" // Standard PowerShell Core on Linux/Mac
    };

    let nomenclature_status = run_command(shell, &["-NoProfile", "-Command", "./scripts/validate-nomenclatura.ps1"]);

    // If pwsh is missing on Linux (e.g. some CI envs), we might need to handle it.
    // However, the protocol demands it. If it fails to run (status false), we fail.
    if !nomenclature_status {
        eprintln!("[ERROR] Nomenclature check failed (or pwsh not found).");
        exit(1);
    }

    // 2. Compilation Check
    println!("[VERIFY-PR-PROTOCOL] 2/3 Compiling Solution...");
    let build_status = run_command("dotnet", &["build", "src/GesFer.Admin.Back.sln"]);
    if !build_status {
        eprintln!("[ERROR] Compilation failed.");
        exit(1);
    }

    // 3. Test Execution
    println!("[VERIFY-PR-PROTOCOL] 3/3 Running Tests...");
    let test_status = run_command("dotnet", &["test", "src/GesFer.Admin.Back.sln"]);
    if !test_status {
        eprintln!("[ERROR] Tests failed.");
        exit(1);
    }

    println!("[VERIFY-PR-PROTOCOL] All checks passed successfully. PR is ready for acceptance.");
}

fn run_command(program: &str, args: &[&str]) -> bool {
    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(s) => s.success(),
        Err(_) => {
            // Siltently fail here to let the caller handle the false return.
            // In a real scenario we might want to log why it failed (e.g. program not found)
            eprintln!("[WARNING] Could not execute command: {}", program);
            false
        }
    }
}
