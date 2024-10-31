use crate::utils::git;

pub struct ProposerBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for ProposerBuildArtifact {
    async fn download(
        &self,
        cfg: &crate::config::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.proposer.release_url,
            &cfg.core.sources.proposer.release_tag,
            &cfg.tree.src.proposer,
        )
        .await
    }

    async fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
