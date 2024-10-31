use crate::utils::git;

pub struct GethBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for GethBuildArtifact {
    async fn download(
        &self,
        cfg: &crate::config::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.geth.release_url,
            &cfg.core.sources.geth.release_tag,
            &cfg.tree.src.geth,
        )
        .await
    }

    async fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
