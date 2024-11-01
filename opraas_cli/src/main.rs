mod commands;
mod config;
mod console;

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
    /// Create new project, template config file and folders
    New { name: String },
    /// Setup a new project
    Setup {},
    /// Compile sources and create docker images for it
    Build { target: String },
    /// Spin up local dev environment
    Dev {},
    /// Deploy your blockchain. Target must be one of: contracts, infra, all
    Deploy { target: String, name: String },
    /// Get details about the current deployment. Target must be one of: contracts, infra
    Inspect { target: String },
    /// Monitor your chain. Target must be one of: onchain, offchain
    Monitor { target: String },
}

#[async_trait]
pub trait Runnable {
  async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // enable logging
    pretty_env_logger::init_custom_env("LOG_LEVEL");

    // Check requirements
    opraas_core::config::requirements::check_requirements().unwrap_or_else(|e| {
        eprintln!("{}", format!("Error: {}", e).bold().red());
        std::process::exit(1);
    });

    // load config
    let config = Config::new_from_root(&std::env::current_dir().unwrap().as_path());

    // run commands
    let args = Args::parse();
    if let Err(e) =  match args.cmd {
        Commands::New { name } => NewCommand { name }.run(&config).await,
        Commands::Setup {} => SetupCommand.run(&config).await,
        Commands::Build { target } => BuildCommand { target }.run(&config).await,
        Commands::Dev {} => DevCommand.run(&config).await,
        Commands::Inspect { target } => InspectCommand { target }.run(&config).await,
        Commands::Monitor { target } => MonitorCommand { target }.run(&config).await,
        Commands::Deploy { target, name } => DeployCommand { target, name }.run(&config).await,
    } {
        eprintln!("{}", format!("Error: {}", e).bold().red());
        std::process::exit(1);
    }
}

