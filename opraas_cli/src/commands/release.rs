use crate::console::{print_info, print_success, print_warning, style_spinner};
use async_trait::async_trait;
use clap::ValueEnum;
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::artifacts::build::{
    BatcherBuildArtifact, BuildArtifact, ContractsBuildArtifact, ExplorerBuildArtifact,
    GethBuildArtifact, NodeBuildArtifact, ProposerBuildArtifact,
};
use std::{sync::Arc, thread, time::Instant};

pub struct ReleaseCommand {
    artifacts: Vec<(&'static str, Arc<dyn BuildArtifact + Send + Sync>)>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ReleaseTargets {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

impl ReleaseCommand {
    pub fn new(target: ReleaseTargets) -> Self {
        let mut artifacts: Vec<(&'static str, Arc<dyn BuildArtifact + Send + Sync>)> = vec![];

        match target {
            ReleaseTargets::Batcher => {
                artifacts.push(("Batcher", Arc::new(BatcherBuildArtifact::new())))
            }
            ReleaseTargets::Node => artifacts.push(("Node", Arc::new(NodeBuildArtifact::new()))),
            ReleaseTargets::Contracts => {
                artifacts.push(("Contracts", Arc::new(ContractsBuildArtifact::new())))
            }
            ReleaseTargets::Explorer => {
                artifacts.push(("Explorer", Arc::new(ExplorerBuildArtifact::new())))
            }
            ReleaseTargets::Proposer => {
                artifacts.push(("Proposer", Arc::new(ProposerBuildArtifact::new())))
            }
            ReleaseTargets::Geth => artifacts.push(("Geth", Arc::new(GethBuildArtifact::new()))),
            ReleaseTargets::All => {
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
impl crate::Runnable for ReleaseCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();
        let core_cfg = Arc::new(cfg.build_core()?);

        // request release name and repository
        print_info("We'll tag your local builds and push them to your repository.");
        print_warning("Make sure you're docker user has push access to the repository");

        let release_name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Input release name (e.g. v0.1.0)")
            .interact_text()
            .unwrap();
        let release_repository: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Input Docker repository url (e.g. docker.io/wakeup-labs) ")
            .interact_text()
            .unwrap();

        // Iterate over the artifacts and build
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let core_cfg = Arc::clone(&core_cfg);
                let release_name = release_name.clone();
                let release_repository = release_repository.clone();
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Releasing {}", name).as_str(),
                );

                thread::spawn(move || -> Result<(), String> {
                    match artifact.release(&core_cfg, &release_name, &release_repository) {
                        Ok(_) => spinner.finish_with_message("Waiting..."),
                        Err(e) => {
                            spinner.finish_with_message(format!("❌ Error setting up {}", name));
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
                Ok(Err(e)) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
                }
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Thread panicked",
                    )))
                }
            }
        }
        m.clear()?;
        print_success(&format!(
            "🎉 Released in {}",
            HumanDuration(started.elapsed())
        ));
        print_info("Yay! Your chain is ready to be deployed!");

        Ok(())
    }
}
