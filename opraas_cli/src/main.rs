mod commands;
mod config;
mod infra;

use build::BuildTargets;
use clap::{Parser, Subcommand};
use colored::Colorize;
use commands::*;
use deploy::DeployTarget;
use dotenv::dotenv;
use infra::console::print_error;
use init::InitTargets;
use inspect::InspectTarget;
use log::{Level, LevelFilter};
use release::ReleaseTargets;

#[derive(Parser)]
#[clap(name = "opruaas")]
#[clap(version = "0.0.3")]
#[clap(about = "Easily deploy and manage rollups with the Optimism stack.", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    /// Suppress logging output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Create new project, template config file and folders
    New { name: String },
    /// Initialize a new project
    Init { target: InitTargets },
    /// Compile sources and create docker images for it
    Build { target: BuildTargets },
    /// Tags and pushes already built docker images to the registry for usage in the deployment
    Release { target: ReleaseTargets },
    /// Spin up local dev environment
    Dev {},
    /// Deploy your blockchain. Target must be one of: contracts, infra, all
    Deploy {
        target: DeployTarget,

        #[arg(long)]
        name: String,

        #[arg(long, default_value_t = false)]
        deterministic_deployer: bool,
    },
    /// Get details about the current deployment. Target must be one of: contracts, infra
    Inspect {
        target: InspectTarget,

        #[arg(long)]
        deployment: String,
    },
    // /// Monitor your chain. Target must be one of: onchain, offchain
    // Monitor { target: MonitorTarget },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();

    let log_level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Off
    };

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Off) // Turn off all logs by default
        .format(|f, record| {
            use std::io::Write;
            let target = record.target();
            let level = match record.level() {
                Level::Trace => "TRACE".red().to_string(),
                Level::Debug => "DEBUG".blue().to_string(),
                Level::Info => "INFO".green().to_string(),
                Level::Warn => "WARN".yellow().to_string(),
                Level::Error => "ERROR".red().to_string(),
            };
            writeln!(f, " {} {} > {}", level, target.bold(), record.args())
        })
        .filter_module("main", log_level)
        .filter_module("opraas_core", log_level)
        .init();

    // run commands
    if let Err(e) = match args.cmd {
        Commands::New { name } => NewCommand::new().run(name),
        Commands::Init { target } => InitCommand::new().run(target),
        Commands::Build { target } => BuildCommand::new().run(target),
        Commands::Release { target } => ReleaseCommand::new().run(target),
        Commands::Dev {} => DevCommand::new().run(),
        Commands::Deploy {
            target,
            name,
            deterministic_deployer,
        } => DeployCommand::new().run(target, name, deterministic_deployer),
        Commands::Inspect { target, deployment } => InspectCommand::new().run(target, deployment),
        // Commands::Monitor { target } => MonitorCommand::new(target).run(&config).await,
    } {
        print_error(&format!("\n\nError: {}\n\n", e));
        std::process::exit(1);
    }
}
