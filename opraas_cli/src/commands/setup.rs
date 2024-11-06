use crate::console::{print_info, print_success, style_spinner};
use async_trait::async_trait;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::artifacts::build::{
    BatcherBuildArtifact, BuildArtifact, ContractsBuildArtifact, ExplorerBuildArtifact,
    GethBuildArtifact, NodeBuildArtifact, ProposerBuildArtifact,
};
use std::{sync::Arc, thread, time::Instant};
use clap::ValueEnum;

pub struct SetupCommand {
    artifacts: Vec<(&'static str, Arc<dyn BuildArtifact + Send + Sync>)>, 
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SetupTargets {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

impl SetupCommand {
    pub fn new(target: SetupTargets) -> Self {
        let mut artifacts: Vec<(&'static str, Arc<dyn BuildArtifact + Send + Sync>)> = vec![];

        match target {
            SetupTargets::Batcher => artifacts.push(("Batcher", Arc::new(BatcherBuildArtifact::new()))),
            SetupTargets::Node => artifacts.push(("Node", Arc::new(NodeBuildArtifact::new()))),
            SetupTargets::Contracts => artifacts.push(("Contracts", Arc::new(ContractsBuildArtifact::new()))),
            SetupTargets::Explorer => artifacts.push(("Explorer", Arc::new(ExplorerBuildArtifact::new()))),
            SetupTargets::Proposer => artifacts.push(("Proposer", Arc::new(ProposerBuildArtifact::new()))),
            SetupTargets::Geth => artifacts.push(("Geth", Arc::new(GethBuildArtifact::new()))),
            SetupTargets::All => {
                artifacts.push(("Batcher", Arc::new(BatcherBuildArtifact::new())));
                artifacts.push(("Node", Arc::new(NodeBuildArtifact::new())));
                artifacts.push(("Contracts", Arc::new(ContractsBuildArtifact::new())));
                artifacts.push(("Explorer", Arc::new(ExplorerBuildArtifact::new())));
                artifacts.push(("Proposer", Arc::new(ProposerBuildArtifact::new())));
                artifacts.push(("Geth", Arc::new(GethBuildArtifact::new())));
            },
        }

        Self { artifacts } 
    }
}

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();
        let core_cfg = Arc::new(cfg.build_core()?);

        print_info("📦 Downloading and preparing artifacts...");

        // Iterate over the artifacts and download
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

                thread::spawn(move || {
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

        print_success(&format!("🎉 Done in {}", HumanDuration(started.elapsed())));

        Ok(())
    }
}
