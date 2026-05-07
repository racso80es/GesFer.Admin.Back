use anyhow::{Context, Result};
use clap::Parser;
use gesfer_capsule::{try_read_capsule_request, write_capsule_response, CapsuleResponse, FeedbackEntry};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const TOOL_ID: &str = "run-tests-local";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    output_json: bool,
    #[arg(long)]
    output_path: Option<String>,
    #[arg(long)]
    skip_prepare: bool,
    #[arg(long)]
    skip_seeds: bool,
    #[arg(long)]
    test_scope: Option<String>,
    #[arg(long)]
    only_tests: bool,
    #[arg(long)]
    e2e_base_url: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct RunTestsLocalInput {
    #[serde(default)]
    skip_prepare: bool,
    #[serde(default)]
    skip_seeds: bool,
    #[serde(default)]
    test_scope: Option<String>,
    #[serde(default)]
    only_tests: bool,
    #[serde(default)]
    output_json: bool,
    #[serde(default)]
    e2e_base_url: Option<String>,
}

fn main() -> Result<()> {
    let start = Instant::now();
    let mut feedback = vec![FeedbackEntry::info("init", "Inicializando run-tests-local")];

    let mut env_input = RunTestsLocalInput::default();
    if let Ok(Some(req)) = try_read_capsule_request() {
        if let Ok(data) = serde_json::from_value::<RunTestsLocalInput>(req.request) {
            env_input = data;
        }
    } else {
        let args = Args::parse();
        env_input.skip_prepare = args.skip_prepare;
        env_input.skip_seeds = args.skip_seeds;
        env_input.test_scope = args.test_scope;
        env_input.only_tests = args.only_tests;
        env_input.output_json = args.output_json;
        env_input.e2e_base_url = args.e2e_base_url;
    }

    let skip_prepare = env_input.only_tests || env_input.skip_prepare;
    let skip_seeds = env_input.only_tests || env_input.skip_seeds;
    let scope = env_input.test_scope.unwrap_or_else(|| "all".to_string());
    let base_url = env_input.e2e_base_url.unwrap_or_else(|| "http://localhost:5010".to_string());

    if !skip_prepare {
        feedback.push(FeedbackEntry::info("prepare", "Invocando prepare-full-env..."));
        let status = Command::new("scripts/tools/prepare-full-env/prepare_full_env.exe").status();
        if status.is_err() || !status.unwrap().success() {
             write_error("prepare_full_env failed", start.elapsed().as_millis() as u64, feedback);
             return Ok(());
        }
    }

    if !skip_seeds {
        feedback.push(FeedbackEntry::info("seeds", "Invocando invoke-mysql-seeds..."));
        let status = Command::new("scripts/tools/invoke-mysql-seeds/invoke_mysql_seeds.exe").status();
        if status.is_err() || !status.unwrap().success() {
             write_error("invoke_mysql_seeds failed", start.elapsed().as_millis() as u64, feedback);
             return Ok(());
        }
    }

    let mut api_job = None;
    if (scope == "all" || scope == "e2e") && !env_input.only_tests {
        feedback.push(FeedbackEntry::info("api", "Iniciando API en background..."));

        let p = Command::new("pwsh")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("Start-Job -ScriptBlock { param($s,$n,$d,$c) & $s -ServiceName $n -WorkingDir $d -Command $c } -ArgumentList 'scripts/run-service-with-log.ps1', 'api', 'src/GesFer.Admin.Back.Api', 'dotnet run'")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        if let Ok(child) = p {
            api_job = Some(child);
            std::thread::sleep(Duration::from_secs(5));
        }

        let health_url = format!("{}/health", base_url.trim_end_matches('/'));
        feedback.push(FeedbackEntry::info("tests", &format!("Esperando API en {}...", health_url)));

        let client = Client::new();
        let mut api_ok = false;
        for _ in 0..60 {
            if let Ok(res) = client.get(&health_url).timeout(Duration::from_secs(2)).send() {
                if res.status().is_success() {
                    api_ok = true;
                    break;
                }
            }
            std::thread::sleep(Duration::from_secs(1));
        }

        if api_ok {
            feedback.push(FeedbackEntry::info("tests", "API lista."));
        } else {
            feedback.push(FeedbackEntry::error("tests", "API no respondio; E2E pueden fallar.", None));
        }
    }

    let target_path = match scope.as_str() {
        "unit" => "src/GesFer.Admin.Back.UnitTests/GesFer.Admin.Back.UnitTests.csproj",
        "integration" => "src/GesFer.Admin.Back.IntegrationTests/GesFer.Admin.Back.IntegrationTests.csproj",
        "e2e" => "src/GesFer.Admin.Back.E2ETests/GesFer.Admin.Back.E2ETests.csproj",
        _ => "src/GesFer.Admin.Back.sln",
    };

    feedback.push(FeedbackEntry::info("build", "Compilando solucion..."));
    let build_status = Command::new("dotnet")
        .arg("build")
        .arg("src/GesFer.Admin.Back.sln")
        .arg("-c")
        .arg("Debug")
        .status();

    if build_status.is_err() || !build_status.unwrap().success() {
         write_error("Build fallo", start.elapsed().as_millis() as u64, feedback);
         return Ok(());
    }

    feedback.push(FeedbackEntry::info("tests", &format!("Ejecutando tests scope={}...", scope)));
    let mut cmd = Command::new("dotnet");
    cmd.arg("test").arg(target_path).arg("--no-build");

    if scope == "e2e" || scope == "all" {
        cmd.env("E2E_BASE_URL", base_url.trim_end_matches('/'));
        cmd.env("E2E_INTERNAL_SECRET", "dev-internal-secret-change-in-production");
    }

    if scope == "e2e" {
        cmd.arg("--filter").arg("Category=E2E");
    }

    let status = cmd.status().context("Failed to execute dotnet test")?;
    let duration = start.elapsed().as_millis() as u64;

    if let Some(mut child) = api_job {
        let _ = Command::new("pwsh")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("Get-Job | Remove-Job -Force")
            .status();
        let _ = child.kill();
    }

    if status.success() {
        feedback.push(FeedbackEntry::info("done", "Tests exitosos"));
        let resp = CapsuleResponse::tool(
            TOOL_ID,
            true,
            0,
            "Tests passed",
            feedback,
            json!({ "duration_ms": duration, "tests_summary": "Passed", "scope": scope }),
            Some(duration)
        );
        let _ = write_capsule_response(&resp);
    } else {
        feedback.push(FeedbackEntry::error("done", "Tests fallaron", None));
        let resp = CapsuleResponse::tool(
            TOOL_ID,
            false,
            1,
            "Tests failed",
            feedback,
            json!({ "duration_ms": duration, "scope": scope }),
            Some(duration)
        );
        let _ = write_capsule_response(&resp);
    }

    Ok(())
}

fn write_error(msg: &str, duration: u64, mut feedback: Vec<FeedbackEntry>) {
    feedback.push(FeedbackEntry::error("error", msg, None));
    let resp = CapsuleResponse::tool(
        TOOL_ID,
        false,
        1,
        msg,
        feedback,
        json!({ "duration_ms": duration }),
        Some(duration)
    );
    let _ = write_capsule_response(&resp);
}