use crate::progress::ProgressTracker;

#[async_trait::async_trait]
pub trait BuildArtifact {
   async fn download<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T) -> Result<(), Box<dyn std::error::Error>>;
   async fn build<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T) -> Result<(), Box<dyn std::error::Error>>;
}
