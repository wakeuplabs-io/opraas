mod commands;
mod config;
use dotenv::dotenv;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Setup a new project
    Setup {},
    /// Build the project
    Build {
        target: String
    },
    /// Deploy the project
    Deploy {
        target: String
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();
    let config = config::config::load_config();

    

    match args.cmd {
        // Commands::Setup{} => commands::setup(&config),
        Commands::Setup{} => println!("config: {:?}", config),
        Commands::Build{target} => commands::build(&config, &target),
        Commands::Deploy{target} => commands::deploy(&config, &target).await,
    }
}
