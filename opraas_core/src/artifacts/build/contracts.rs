pub struct ContractsBuildArtifact;

#[async_trait::async_trait]
impl crate::artifacts::build::BuildArtifact for ContractsBuildArtifact {
    async fn download(
        &self,
        _cfg: &crate::config::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
