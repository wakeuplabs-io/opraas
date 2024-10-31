use crate::{git, progress::ProgressTracker};

pub struct BatcherBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for BatcherBuildArtifact {
    async fn download<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.batcher.base_url,
            &cfg.core.sources.batcher.release_tag,
            &cfg.tree.src.batcher,
            progress,
        )
        .await
    }

    async fn build<T: ProgressTracker>(&self, _cfg: &crate::config::Config, _progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
