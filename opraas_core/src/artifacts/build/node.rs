use crate::{git, progress::ProgressTracker};
pub struct NodeBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for NodeBuildArtifact {
    async fn download<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.node.base_url,
            &cfg.core.sources.node.release_tag,
            &cfg.tree.src.node,
            progress
            
        )
        .await
    }

    async fn build<T: ProgressTracker>(&self, _cfg: &crate::config::Config, _progress: &T) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
