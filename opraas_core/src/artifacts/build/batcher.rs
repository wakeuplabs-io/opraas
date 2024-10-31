use crate::utils::git;

pub struct BatcherBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for BatcherBuildArtifact {
    async fn download(
        &self,
        cfg: &crate::config::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.batcher.release_url,
            &cfg.core.sources.batcher.release_tag,
            &cfg.tree.src.batcher,
        )
        .await
    }

    async fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
