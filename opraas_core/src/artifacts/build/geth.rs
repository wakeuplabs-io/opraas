use crate::{git, progress::ProgressTracker};

pub struct GethBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for GethBuildArtifact {
    async fn download<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.geth.release_url,
            &cfg.core.sources.geth.release_tag,
            &cfg.tree.src.geth,
            progress
        )
        .await
    }

    async fn build<T: ProgressTracker>(&self, _cfg: &crate::config::Config, _progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
