

#[async_trait::async_trait]
pub trait BuildArtifact {
    async fn download(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
    async fn build(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
}
