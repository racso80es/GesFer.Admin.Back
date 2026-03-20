//! verify-pr-protocol — TTY (logs) o JSON stdin (respuesta capsule-json-io).

use std::process::{exit, Command};
use std::time::Instant;

use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};

const SKILL_ID: &str = "verify-pr-protocol";

fn main() {
    let agent = match try_read_capsule_request() {
        Ok(None) => false,
        Ok(Some(_)) => true,
        Err(e) => {
            eprintln!("{}", e);
            exit(2);
        }
    };
    let start = Instant::now();

    let mut feedback = Vec::new();

    if agent {
        feedback.push(FeedbackEntry::info("init", "verify-pr-protocol (JSON)"));
    } else {
        println!("[VERIFY-PR-PROTOCOL] Starting PR Acceptance Protocol...");
    }

    if !agent {
        println!("[VERIFY-PR-PROTOCOL] 1/3 Checking Nomenclature...");
    } else {
        feedback.push(FeedbackEntry::info("nomenclature", "Checking nomenclature"));
    }
    let shell = if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "pwsh"
    };
    let nomenclature_status =
        run_command(shell, &["-NoProfile", "-Command", "./scripts/validate-nomenclatura.ps1"]);

    if !nomenclature_status {
        if agent {
            feedback.push(FeedbackEntry::error(
                "nomenclature",
                "Nomenclature check failed",
                None,
            ));
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                1,
                "Nomenclature check failed",
                feedback,
                serde_json::json!({}),
                Some(start.elapsed().as_millis() as u64),
            );
            let _ = write_capsule_response(&res);
        } else {
            eprintln!("[ERROR] Nomenclature check failed (or pwsh not found).");
        }
        exit(1);
    }

    if !agent {
        println!("[VERIFY-PR-PROTOCOL] 2/3 Compiling Solution...");
    } else {
        feedback.push(FeedbackEntry::info("build", "dotnet build"));
    }
    let build_status = run_command(
        "dotnet",
        &["build", "src/GesFer.Admin.Back.sln"],
    );
    if !build_status {
        if agent {
            feedback.push(FeedbackEntry::error("build", "Compilation failed", None));
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                2,
                "Compilation failed",
                feedback,
                serde_json::json!({}),
                Some(start.elapsed().as_millis() as u64),
            );
            let _ = write_capsule_response(&res);
        } else {
            eprintln!("[ERROR] Compilation failed.");
        }
        exit(1);
    }

    if !agent {
        println!("[VERIFY-PR-PROTOCOL] 3/3 Running Tests...");
    } else {
        feedback.push(FeedbackEntry::info("tests", "dotnet test"));
    }
    let test_status = run_command(
        "dotnet",
        &["test", "src/GesFer.Admin.Back.sln"],
    );
    if !test_status {
        if agent {
            feedback.push(FeedbackEntry::error("tests", "Tests failed", None));
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                3,
                "Tests failed",
                feedback,
                serde_json::json!({}),
                Some(start.elapsed().as_millis() as u64),
            );
            let _ = write_capsule_response(&res);
        } else {
            eprintln!("[ERROR] Tests failed.");
        }
        exit(1);
    }

    if agent {
        feedback.push(FeedbackEntry::info("done", "All checks passed"));
        let res = CapsuleResponse::skill(
            SKILL_ID,
            true,
            0,
            "PR protocol OK",
            feedback,
            serde_json::json!({ "checks": ["nomenclature", "build", "test"] }),
            Some(start.elapsed().as_millis() as u64),
        );
        let _ = write_capsule_response(&res);
    } else {
        println!("[VERIFY-PR-PROTOCOL] All checks passed successfully. PR is ready for acceptance.");
    }
}

fn run_command(program: &str, args: &[&str]) -> bool {
    match Command::new(program).args(args).status() {
        Ok(s) => s.success(),
        Err(_) => {
            eprintln!("[WARNING] Could not execute command: {}", program);
            false
        }
    }
}
