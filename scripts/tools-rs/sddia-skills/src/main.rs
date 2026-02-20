use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sddia-skills")]
#[command(about = "SddIA Skills Runner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Executes a specific skill
    Run {
        /// The ID/Name of the skill to execute
        skill_id: String,
    },
    /// Lists available skills (placeholder)
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { skill_id } => {
            println!("Executing skill: {}", skill_id);
            // In a real implementation, this would look up the skill definition
            // and execute the corresponding logic or script.
            // For now, it's a placeholder to satisfy the requirement.
            println!("(Simulation) Skill '{}' logic would run here.", skill_id);
        }
        Commands::List => {
            println!("Available skills (placeholder):");
            println!(" - iniciar-rama");
            println!(" - feature");
        }
    }
}
