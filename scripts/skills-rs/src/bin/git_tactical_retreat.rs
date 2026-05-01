//! Skill git-tactical-retreat — checkout -- <path> y/o reset --hard + clean -fd (con guardas).

use std::process::Command;

use clap::Parser;
use gesfer_skills::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use serde::Deserialize;

const SKILL_ID: &str = "git-tactical-retreat";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct RetreatRequest {
    #[serde(default)]
    target_path: Option<String>,
    #[serde(default)]
    hard_reset: Option<bool>,
    #[serde(default)]
    confirm_destructive: Option<bool>,
}

#[derive(Debug, Parser)]
#[command(name = "git_tactical_retreat", version, about = "Revertir cambios: checkout -- path y/o reset --hard + clean -fd")]
struct CliArgs {
    #[arg(long = "target-path")]
    target_path: Option<String>,
    #[arg(long = "hard-reset")]
    hard_reset: bool,
    #[arg(long = "confirm-destructive")]
    confirm_destructive: bool,
}

fn main() {
    let start = std::time::Instant::now();

    if let Ok(Some(req)) = try_read_capsule_request() {
        let body: RetreatRequest = serde_json::from_value(req.request.clone()).unwrap_or_default();
        let hard_reset = body.hard_reset.unwrap_or(false);
        let confirm = body.confirm_destructive.unwrap_or(false);
        let (res, exit_code) = run(body.target_path, hard_reset, confirm, start);
        let _ = write_capsule_response(&res);
        std::process::exit(exit_code);
    }

    let args = CliArgs::parse();
    let (res, exit_code) = run(args.target_path, args.hard_reset, args.confirm_destructive, start);
    let _ = write_capsule_response(&res);
    std::process::exit(exit_code);
}

fn run(
    target_path: Option<String>,
    hard_reset: bool,
    confirm_destructive: bool,
    start: std::time::Instant,
) -> (CapsuleResponse, i32) {
    let mut feedback: Vec<FeedbackEntry> = vec![];

    if hard_reset && !confirm_destructive {
        let res = CapsuleResponse::skill(
            SKILL_ID,
            false,
            2,
            "Operación destructiva requiere confirm_destructive=true",
            vec![FeedbackEntry::error(
                "validate",
                "hard_reset requiere confirm_destructive",
                None,
            )],
            serde_json::json!({
                "checkout": { "executed": false, "exitCode": null, "output": "" },
                "resetHard": { "executed": false, "exitCode": null, "output": "" },
                "cleanFd": { "executed": false, "exitCode": null, "output": "" }
            }),
            Some(start.elapsed().as_millis() as u64),
        );
        return (res, 2);
    }

    let mut checkout = StepOut::skipped();
    let mut reset_hard = StepOut::skipped();
    let mut clean_fd = StepOut::skipped();

    if let Some(p) = target_path
        .as_ref()
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
    {
        feedback.push(FeedbackEntry::info("git", "git checkout -- <path>"));
        let out = run_git(&["checkout", "--", p.as_str()]);
        checkout = StepOut::executed(out.exit_code, out.combined);
        if checkout.exit_code() != 0 {
            feedback.push(FeedbackEntry::error(
                "git",
                "git checkout -- falló",
                Some(&checkout.output),
            ));
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                checkout.exit_code(),
                "Checkout falló",
                feedback,
                serde_json::json!({
                    "checkout": checkout.to_json(),
                    "resetHard": reset_hard.to_json(),
                    "cleanFd": clean_fd.to_json()
                }),
                Some(start.elapsed().as_millis() as u64),
            );
            return (res, checkout.exit_code());
        }
    }

    if hard_reset {
        feedback.push(FeedbackEntry::warning("git", "Ejecutando hard reset", None));
        let out_reset = run_git(&["reset", "--hard", "HEAD"]);
        reset_hard = StepOut::executed(out_reset.exit_code, out_reset.combined);
        if reset_hard.exit_code() != 0 {
            feedback.push(FeedbackEntry::error(
                "git",
                "git reset --hard falló",
                Some(&reset_hard.output),
            ));
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                reset_hard.exit_code(),
                "Reset --hard falló",
                feedback,
                serde_json::json!({
                    "checkout": checkout.to_json(),
                    "resetHard": reset_hard.to_json(),
                    "cleanFd": clean_fd.to_json()
                }),
                Some(start.elapsed().as_millis() as u64),
            );
            return (res, reset_hard.exit_code());
        }

        let out_clean = run_git(&["clean", "-fd"]);
        clean_fd = StepOut::executed(out_clean.exit_code, out_clean.combined);
        if clean_fd.exit_code() != 0 {
            feedback.push(FeedbackEntry::error(
                "git",
                "git clean -fd falló",
                Some(&clean_fd.output),
            ));
            let res = CapsuleResponse::skill(
                SKILL_ID,
                false,
                clean_fd.exit_code(),
                "Clean -fd falló",
                feedback,
                serde_json::json!({
                    "checkout": checkout.to_json(),
                    "resetHard": reset_hard.to_json(),
                    "cleanFd": clean_fd.to_json()
                }),
                Some(start.elapsed().as_millis() as u64),
            );
            return (res, clean_fd.exit_code());
        }
    }

    let res = CapsuleResponse::skill(
        SKILL_ID,
        true,
        0,
        "Retreat completado",
        feedback,
        serde_json::json!({
            "checkout": checkout.to_json(),
            "resetHard": reset_hard.to_json(),
            "cleanFd": clean_fd.to_json()
        }),
        Some(start.elapsed().as_millis() as u64),
    );
    (res, 0)
}

#[derive(Clone)]
struct StepOut {
    executed: bool,
    exit_code: Option<i32>,
    output: String,
}

impl StepOut {
    fn skipped() -> Self {
        Self {
            executed: false,
            exit_code: None,
            output: String::new(),
        }
    }

    fn executed(exit_code: i32, output: String) -> Self {
        Self {
            executed: true,
            exit_code: Some(exit_code),
            output,
        }
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "executed": self.executed,
            "exitCode": self.exit_code,
            "output": self.output
        })
    }

    fn exit_code(&self) -> i32 {
        self.exit_code.unwrap_or(0)
    }
}

struct CmdOut {
    exit_code: i32,
    combined: String,
}

fn run_git(args: &[&str]) -> CmdOut {
    let output = Command::new("git").args(args).output();
    match output {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let code = o.status.code().unwrap_or(-1);
            CmdOut {
                exit_code: code,
                combined: format!("{}\n{}", out, err).trim().to_string(),
            }
        }
        Err(e) => CmdOut {
            exit_code: 1,
            combined: e.to_string(),
        },
    }
}

