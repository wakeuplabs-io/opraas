mod commands;
mod config;
use clap::{Parser, Subcommand};
use colored::*;
use dotenv::dotenv;

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

    match args.cmd {
        Commands::Setup {} => commands::setup(&config),
        Commands::Build { target } => commands::build(&config, &target),
        Commands::Deploy { target, name } => commands::deploy(&config, &target, &name).await,
    }
}
