use crate::console::style_spinner;
use async_trait::async_trait;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::artifacts::build::{
    BatcherBuildArtifact, BuildArtifact, ContractsBuildArtifact, ExplorerBuildArtifact,
    GethBuildArtifact, NodeBuildArtifact, ProposerBuildArtifact,
};
use std::{sync::Arc, thread, time::Instant};
use clap::ValueEnum;

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
            BuildTargets::Batcher => artifacts.push(("Batcher", Arc::new(BatcherBuildArtifact::new()))),
            BuildTargets::Node => artifacts.push(("Node", Arc::new(NodeBuildArtifact::new()))),
            BuildTargets::Contracts => artifacts.push(("Contracts", Arc::new(ContractsBuildArtifact::new()))),
            BuildTargets::Explorer => artifacts.push(("Explorer", Arc::new(ExplorerBuildArtifact::new()))),
            BuildTargets::Proposer => artifacts.push(("Proposer", Arc::new(ProposerBuildArtifact::new()))),
            BuildTargets::Geth => artifacts.push(("Geth", Arc::new(GethBuildArtifact::new()))),
            BuildTargets::All => {
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
impl crate::Runnable for BuildCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();
        let core_cfg = Arc::new(cfg.build_core()?);

        println!("📦 Downloading and preparing artifacts...");

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
                    if let Err(e) = artifact.build(&core_cfg) {
                        eprintln!("Error setting up {}: {}", name, e);
                    }

                    spinner.finish_with_message("Waiting...");
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            let _ = handle.join();
        }
        m.clear().unwrap();

        println!("🎉 Done in {}", HumanDuration(started.elapsed()));

        Ok(())
    }
}
