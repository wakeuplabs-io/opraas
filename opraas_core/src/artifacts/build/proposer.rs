use crate::{git, progress::ProgressTracker};

pub struct ProposerBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for ProposerBuildArtifact {
    async fn download<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.proposer.release_url,
            &cfg.core.sources.proposer.release_tag,
            &cfg.tree.src.proposer,
            progress
        )
        .await
    }

    async fn build<T: ProgressTracker>(&self, _cfg: &crate::config::Config, _progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
