//! SddIA Skills Runner.
//! Lazy-loading: List usa solo spec.json; Show carga spec.md solo cuando se necesita contexto completo.

use clap::{Parser, Subcommand};
use serde_json::Value;
use std::fs;

const SKILLS_PATH: &str = "SddIA/skills";

/// Resuelve la ruta base del repo (donde está SddIA/)
fn find_repo_root() -> Option<std::path::PathBuf> {
    let mut current = std::env::current_dir().ok()?;
    loop {
        let sddia = current.join(SKILLS_PATH);
        if sddia.exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Lista skills leyendo SOLO spec.json (lazy-loading: no carga spec.md)
fn list_skills_json_only() -> Result<Vec<String>, String> {
    let root = find_repo_root().ok_or("No se encontró SddIA/skills (ejecutar desde raíz del repo)")?;
    let skills_dir = root.join(SKILLS_PATH);
    let mut skills = Vec::new();
    for entry in fs::read_dir(&skills_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            let spec_json = path.join("spec.json");
            if spec_json.exists() {
                if let Some(name) = path.file_name() {
                    skills.push(name.to_string_lossy().to_string());
                }
            }
        }
    }
    skills.sort();
    Ok(skills)
}

/// Carga contexto completo: spec.json + spec.md (solo cuando se necesita)
fn load_full_context(skill_id: &str) -> Result<(Value, String), String> {
    let root = find_repo_root().ok_or("No se encontró SddIA/skills")?;
    let skill_dir = root.join(SKILLS_PATH).join(skill_id);
    let spec_json_path = skill_dir.join("spec.json");
    let spec_md_path = skill_dir.join("spec.md");
    if !spec_json_path.exists() {
        return Err(format!("Skill '{}' no encontrado (no spec.json)", skill_id));
    }
    let json_str = fs::read_to_string(&spec_json_path).map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
    let md_content = if spec_md_path.exists() {
        fs::read_to_string(&spec_md_path).unwrap_or_default()
    } else {
        String::new()
    };
    Ok((json, md_content))
}

#[derive(Parser)]
#[command(name = "sddia-skills")]
#[command(about = "SddIA Skills Runner. Lazy-loading: List usa solo spec.json.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lista skills (lee SOLO spec.json, no spec.md - lazy-loading)
    List,
    /// Muestra contexto completo de un skill (carga spec.json + spec.md)
    Show {
        /// ID del skill
        skill_id: String,
    },
    /// Ejecuta un skill (placeholder)
    Run {
        /// ID del skill
        skill_id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            match list_skills_json_only() {
                Ok(skills) => {
                    println!("Skills (solo spec.json cargado, sin spec.md):");
                    for s in skills {
                        println!("  - {}", s);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Show { skill_id } => {
            match load_full_context(skill_id) {
                Ok((json, md)) => {
                    println!("=== spec.json ===\n{}", serde_json::to_string_pretty(&json).unwrap_or_default());
                    if !md.is_empty() {
                        println!("\n=== spec.md (primeras 20 líneas) ===\n{}", md.lines().take(20).collect::<Vec<_>>().join("\n"));
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Run { skill_id } => {
            println!("(Simulation) Skill '{}' logic would run here.", skill_id);
        }
    }
}
