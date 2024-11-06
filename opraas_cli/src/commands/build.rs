use crate::console::{print_info, print_success, style_spinner};
use async_trait::async_trait;
use clap::ValueEnum;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::artifacts::build::{
    BatcherBuildArtifact, BuildArtifact, ContractsBuildArtifact, ExplorerBuildArtifact,
    GethBuildArtifact, NodeBuildArtifact, ProposerBuildArtifact,
};
use std::{sync::Arc, thread, time::Instant};

pub struct BuildCommand {
    artifacts: Vec<(&'static str, Arc<dyn BuildArtifact + Send + Sync>)>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BuildTargets {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

impl BuildCommand {
    pub fn new(target: BuildTargets) -> Self {
        let mut artifacts: Vec<(&'static str, Arc<dyn BuildArtifact + Send + Sync>)> = vec![];

        match target {
            BuildTargets::Batcher => {
                artifacts.push(("Batcher", Arc::new(BatcherBuildArtifact::new())))
            }
            BuildTargets::Node => artifacts.push(("Node", Arc::new(NodeBuildArtifact::new()))),
            BuildTargets::Contracts => {
                artifacts.push(("Contracts", Arc::new(ContractsBuildArtifact::new())))
            }
            BuildTargets::Explorer => {
                artifacts.push(("Explorer", Arc::new(ExplorerBuildArtifact::new())))
            }
            BuildTargets::Proposer => {
                artifacts.push(("Proposer", Arc::new(ProposerBuildArtifact::new())))
            }
            BuildTargets::Geth => artifacts.push(("Geth", Arc::new(GethBuildArtifact::new()))),
            BuildTargets::All => {
                artifacts.push(("Batcher", Arc::new(BatcherBuildArtifact::new())));
                artifacts.push(("Node", Arc::new(NodeBuildArtifact::new())));
                artifacts.push(("Contracts", Arc::new(ContractsBuildArtifact::new())));
                artifacts.push(("Explorer", Arc::new(ExplorerBuildArtifact::new())));
                artifacts.push(("Proposer", Arc::new(ProposerBuildArtifact::new())));
                artifacts.push(("Geth", Arc::new(GethBuildArtifact::new())));
            }
        }

        Self { artifacts }
    }
}

#[async_trait]
impl crate::Runnable for BuildCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();
        let core_cfg = Arc::new(cfg.build_core()?);

        // Iterate over the artifacts and build
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let core_cfg = Arc::clone(&core_cfg);
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Preparing {}", name).as_str(),
                );

                thread::spawn(move || -> Result<(), String> {
                    match artifact.build(&core_cfg) {
                        Ok(_) => spinner.finish_with_message("Waiting..."),
                        Err(e) => {
                            spinner.finish_with_message(format!("❌ Error setting up {}", name));
                            return Err(e.to_string());
                        },
                    }
                    Ok(())
              })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            match handle.join() {
                Ok(Ok(_)) => {},
                Ok(Err(e)) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))), 
                Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Thread panicked"))), // Panic
            }
        }
        m.clear()?;
        print_success(&format!("🎉 Built in {}", HumanDuration(started.elapsed())));

        // check if we need to push and exit if not
        if !self.artifacts.iter().any(|&(_, ref artifact)| artifact.needs_push(&core_cfg)) {
            return Ok(())
        }

        let push_images = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to push the images? (Required for deployment)")
            .interact()
            .unwrap();
        if !push_images {
            print_info("🎉 Cool, you're done then! You can push later with docker tag and push");
            return Ok(());
        }

        let repository: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter your docker repository url: (example ...amazonaws.com/wakeuplabs)")
            .interact_text()
            .unwrap();

        // Iterate over the artifacts and push
        let m = MultiProgress::new();
        let push_started = Instant::now();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let core_cfg = Arc::clone(&core_cfg);
                let repository_str = repository.clone();
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Pushing {}", name).as_str(),
                );

                thread::spawn(move || -> Result<(), String> {
                    if !artifact.needs_push(&core_cfg) {
                        spinner.finish_with_message("Skipping...");
                        return Ok(());
                    }

                    match artifact.push(&core_cfg, &repository_str) {
                        Ok(_) => spinner.finish_with_message("Waiting..."),
                        Err(e) => {
                            spinner.finish_with_message(format!("❌ Error pushing {}", name));
                            return Err(e.to_string());
                        },
                    }
                    Ok(())
              })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            match handle.join() {
                Ok(Ok(_)) => {},
                Ok(Err(e)) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))), 
                Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Thread panicked"))), // Panic
            }
        }
        m.clear()?;

        print_success(&format!("🎉 Pushed in {}", HumanDuration(push_started.elapsed())));

        Ok(())
    }
}