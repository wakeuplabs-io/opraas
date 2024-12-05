use crate::{
    config::{SystemRequirementsChecker, TSystemRequirementsChecker, BIN_NAME, DOCKER_REQUIREMENT, GIT_REQUIREMENT},
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
    system_requirements_checker: Box<dyn TSystemRequirementsChecker>,
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
        let project = Project::new_from_cwd().unwrap();
        let config = CoreConfig::new_from_toml(&project.config).unwrap();

        let artifacts_factory = ArtifactFactory::new(&project, &config);
        let artifacts = match target {
            BuildTargets::All => artifacts_factory.get_all(),
            BuildTargets::Batcher => vec![artifacts_factory.get(ArtifactKind::Batcher)],
            BuildTargets::Node => vec![artifacts_factory.get(ArtifactKind::Node)],
            BuildTargets::Contracts => vec![artifacts_factory.get(ArtifactKind::Contracts)],
            BuildTargets::Proposer => vec![artifacts_factory.get(ArtifactKind::Proposer)],
            BuildTargets::Geth => vec![artifacts_factory.get(ArtifactKind::Geth)],
        };

        Self {
            artifacts,
            system_requirements_checker: Box::new(SystemRequirementsChecker::new()),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.system_requirements_checker
            .check(vec![GIT_REQUIREMENT, DOCKER_REQUIREMENT])?;

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

        println!(
            "\n{title}\n\n\
            - {bin} {release_cmd}\n\
            \tPublishes artifacts to registry for consumption in dev and deploy.\n\n\
            - {bin} {dev_cmd}\n\
            \tTry your artifacts locally without spending any resources.\n\n\
            - {bin} {deploy_cmd}\n\
            \tUse your artifacts to create contracts deployments or whole infra.\n",
            title = "What's Next?".bright_white().bold(),
            bin = BIN_NAME.blue(),
            release_cmd = "release [contracts|node|etc...]".blue(),
            dev_cmd = "dev".blue(),
            deploy_cmd = "deploy [contracts|infra|all] --name <deployment_name>".blue()
        );

        Ok(())
    }
}
