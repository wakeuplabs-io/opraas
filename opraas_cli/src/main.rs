mod commands;
mod config;
use clap::{Parser, Subcommand};
use colored::*;
use commands::*;
use config::Config;
use dotenv::dotenv;
use async_trait::async_trait;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Setup a new project
    Setup {},
    /// Build the project
    Build { target: String },
    /// Deploy the project
    Deploy { target: String, name: String },
    /// Version
    Version {},
}

#[async_trait]
pub trait Runnable {
  async fn run(&self, cfg: &Config) -> Result<(), Box<dyn std::error::Error>>;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();
    let config = config::load_config();

    // Check requirements
    opraas_core::config::requirements::check_requirements().unwrap_or_else(|e| {
        eprintln!("{}", format!("Panic: {}", e).bold().red());
        std::process::exit(1);
    });

    if let Err(e) =  match args.cmd {
        Commands::Version {} => VersionCommand.run(&config).await,
        Commands::Setup {} => SetupCommand.run(&config).await,
        Commands::Build { target } => BuildCommand { target }.run(&config).await,
        Commands::Deploy { target, name } => DeployCommand { target, name }.run(&config).await,
    } {
        eprintln!("{}", format!("Panic: {}", e).bold().red());
        std::process::exit(1);
    }
}
