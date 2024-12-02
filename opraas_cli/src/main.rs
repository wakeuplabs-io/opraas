mod commands;
mod config;
mod console;
mod utils;

use build::BuildTargets;
use colored::Colorize;
use deploy::DeployTarget;
use init::InitTargets;
use inspect::InspectTarget;
use log::{Level, LevelFilter};
use release::ReleaseTargets;
pub use utils::*;

use clap::{Parser, Subcommand};
use commands::*;
use config::{Comparison, Requirement, SystemRequirementsChecker, TSystemRequirementsChecker};
use console::print_error;
use dotenv::dotenv;
use semver::Version;

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

    // parse args
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

    // Check requirements
    SystemRequirementsChecker::new()
        .check(vec![
            Requirement {
                program: "docker",
                version_arg: "-v",
                required_version: Version::parse("24.0.0").unwrap(),
                required_comparator: Comparison::GreaterThanOrEqual,
            },
            Requirement {
                program: "kubectl",
                version_arg: "version",
                required_version: Version::parse("1.28.0").unwrap(),
                required_comparator: Comparison::GreaterThanOrEqual,
            },
            Requirement {
                program: "helm",
                version_arg: "version",
                required_version: Version::parse("3.0.0").unwrap(),
                required_comparator: Comparison::GreaterThanOrEqual,
            },
            Requirement {
                program: "terraform",
                version_arg: "-v",
                required_version: Version::parse("1.9.8").unwrap(),
                required_comparator: Comparison::GreaterThanOrEqual,
            },
            Requirement {
                program: "git",
                version_arg: "--version",
                required_version: Version::parse("2.0.0").unwrap(),
                required_comparator: Comparison::GreaterThanOrEqual,
            },
        ])
        .unwrap_or_else(|e| {
            print_error(&format!("\n\nError: {}\n\n", e));
            std::process::exit(1);
        });

    // run commands
    if let Err(e) = match args.cmd {
        Commands::New { name } => NewCommand::new().run(name),
        Commands::Init { target } => InitCommand::new(target).run(),
        Commands::Build { target } => BuildCommand::new(target).run(),
        Commands::Release { target } => ReleaseCommand::new(target).run(),
        Commands::Dev {} => DevCommand::new().run(),
        Commands::Deploy { target, name } => DeployCommand::new().run(target, name),
        Commands::Inspect { target, deployment } => InspectCommand::new().run(target, deployment),
        // Commands::Monitor { target } => MonitorCommand::new(target).run(&config).await,
    } {
        print_error(&format!("\n\nError: {}\n\n", e));
        std::process::exit(1);
    }
}
