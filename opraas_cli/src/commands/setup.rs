use crate::console::ConsoleProgressTracker;
use async_trait::async_trait;
use opraas_core::artifacts::build::{
    BatcherBuildArtifact, BuildArtifact, ContractsBuildArtifact, ExplorerBuildArtifact,
    GethBuildArtifact, NodeBuildArtifact, ProposerBuildArtifact,
};

pub struct SetupCommand;

enum BuildArtifacts {
    Batcher(BatcherBuildArtifact),
    Node(NodeBuildArtifact),
    Contracts(ContractsBuildArtifact),
    Explorer(ExplorerBuildArtifact),
    Proposer(ProposerBuildArtifact),
    Geth(GethBuildArtifact),
}

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let core_cfg = cfg.build_core()?;

        let artifacts = vec![
            (
                "Batcher",
                cfg.tree.src.batcher.exists(),
                BuildArtifacts::Batcher(BatcherBuildArtifact),
            ),
            (
                "Node",
                cfg.tree.src.node.exists(),
                BuildArtifacts::Node(NodeBuildArtifact),
            ),
            (
                "Contracts",
                cfg.tree.src.contracts.exists(),
                BuildArtifacts::Contracts(ContractsBuildArtifact),
            ),
            (
                "Explorer",
                cfg.tree.src.explorer.exists(),
                BuildArtifacts::Explorer(ExplorerBuildArtifact),
            ),
            (
                "Proposer",
                cfg.tree.src.proposer.exists(),
                BuildArtifacts::Proposer(ProposerBuildArtifact),
            ),
            (
                "Geth",
                cfg.tree.src.geth.exists(),
                BuildArtifacts::Geth(GethBuildArtifact),
            ),
        ];

        // Iterate over the artifacts and download
        for (name, exists, artifact) in artifacts {
            let progress = ConsoleProgressTracker::new(format!("⏳ Downloading {}", name).as_str());

            // Check if the artifact's path exists before downloading
            if !exists {
                match artifact {
                    BuildArtifacts::Batcher(batcher) => {
                        batcher.download(&core_cfg, &progress).await?;
                    }
                    BuildArtifacts::Node(node) => {
                        node.download(&core_cfg, &progress).await?;
                    }
                    BuildArtifacts::Contracts(contracts) => {
                        contracts.download(&core_cfg, &progress).await?;
                    }
                    BuildArtifacts::Explorer(explorer) => {
                        explorer.download(&core_cfg, &progress).await?;
                    }
                    BuildArtifacts::Proposer(proposer) => {
                        proposer.download(&core_cfg, &progress).await?;
                    }
                    BuildArtifacts::Geth(geth) => {
                        geth.download(&core_cfg, &progress).await?;
                    }
                }
            }

            progress.finish(format!("✅ {} ready", name).as_str());
        }

        Ok(())
    }
}
