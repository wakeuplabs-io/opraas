use crate::console::ConsoleProgressTracker;
use async_trait::async_trait;
use opraas_core::artifacts::build::{
    BatcherBuildArtifact, BuildArtifact, ContractsBuildArtifact, ExplorerBuildArtifact,
    GethBuildArtifact, NodeBuildArtifact, ProposerBuildArtifact,
};

pub struct SetupCommand;

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let core_cfg = cfg.build_core()?;

        let artifacts: Vec<( &str, Box<dyn BuildArtifact>)> = vec![
            ("Batcher", Box::new(BatcherBuildArtifact::new())),
            ("Node", Box::new(NodeBuildArtifact::new())),
            ("Contracts", Box::new(ContractsBuildArtifact::new())),
            ("Explorer", Box::new(ExplorerBuildArtifact::new())),
            ("Proposer", Box::new(ProposerBuildArtifact::new())),
            ("Geth", Box::new(GethBuildArtifact::new())),
        ];

        // Iterate over the artifacts and download
        for (name, artifact) in artifacts {
            let progress = ConsoleProgressTracker::new(format!("⏳ Preparing {}", name).as_str());

            artifact.setup(&core_cfg, &progress)?;

            progress.finish(format!("✅ {} ready", name).as_str());
        }

        Ok(())
    }
}
