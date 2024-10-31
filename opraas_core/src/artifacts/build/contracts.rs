use crate::utils::git;

pub struct ContractsBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for ContractsBuildArtifact {
    async fn download(
        &self,
        cfg: &crate::config::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.contracts.release_url,
            &cfg.core.sources.contracts.release_tag,
            &cfg.tree.src.contracts,
        )
        .await
    }

    async fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
