use crate::utils::git;

pub struct NodeBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for NodeBuildArtifact {
    async fn download(
        &self,
        cfg: &crate::config::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(
            &cfg.core.sources.node.release_url,
            &cfg.core.sources.node.release_tag,
            &cfg.tree.src.node,
        )
        .await
    }

    async fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
