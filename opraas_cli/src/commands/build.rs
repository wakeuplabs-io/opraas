use crate::{
    config::{get_config_path, BIN_NAME},
    console::{print_error, style_spinner},
};
use colored::*;
use indicatif::{HumanDuration, ProgressBar};
use opraas_core::{
    application::build::{ArtifactBuilderService, TArtifactBuilderService},
    config::CoreConfig,
    domain::{Artifact, ArtifactFactory, ArtifactKind, Project},
};
use std::{sync::Arc, thread, time::Instant};

pub struct BuildCommand {
    artifacts: Vec<Arc<Artifact>>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BuildTargets {
    Batcher,
    Node,
    Contracts,
    Proposer,
    Geth,
    All,
}

// implementations ================================================

impl BuildCommand {
    pub fn new(target: BuildTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        let artifacts_factory = ArtifactFactory::new(&project, &config);
        let artifacts = match target {
            BuildTargets::All => artifacts_factory.get_all(),
            BuildTargets::Batcher => vec![artifacts_factory.get(ArtifactKind::Batcher)],
            BuildTargets::Node => vec![artifacts_factory.get(ArtifactKind::Node)],
            BuildTargets::Contracts => vec![artifacts_factory.get(ArtifactKind::Contracts)],
            BuildTargets::Proposer => vec![artifacts_factory.get(ArtifactKind::Proposer)],
            BuildTargets::Geth => vec![artifacts_factory.get(ArtifactKind::Geth)],
        };

        Self { artifacts }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();

        let build_spinner = style_spinner(
            ProgressBar::new_spinner(),
            &format!(
                "⏳ Building {}...",
                self.artifacts
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );

        // Iterate over the artifacts and build
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&ref artifact| {
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership

                thread::spawn(move || -> Result<(), String> {
                    match ArtifactBuilderService::new().build(&artifact) {
                        Ok(_) => {}
                        Err(e) => {
                            print_error(&format!("❌ Error building {}", artifact));
                            return Err(e.to_string());
                        }
                    }
                    Ok(())
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            match handle.join() {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))),
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Thread panicked",
                    )))
                }
            }
        }

        build_spinner.finish_with_message(format!("✔️ Built in {}", HumanDuration(started.elapsed())));

        // print instructions

        println!("\n{}\n", "What's Next?".bright_white().bold());

        println!(
            "  {} {}",
            BIN_NAME.blue(),
            "release [contracts|node|etc...]".blue()
        );
        println!("    Publishes artifacts to registry for consumption in dev and deploy.\n");

        println!("  {} {}", BIN_NAME.blue(), "dev".blue());
        println!("    Try your artifacts locally without spending any resources.\n");

        println!(
            "  {} {}",
            BIN_NAME.blue(),
            "deploy [contracts|infra|all] --name <deployment_name>".blue()
        );
        println!("    Use your artifacts to create contracts deployments or whole infra.\n");

        Ok(())
    }
}
