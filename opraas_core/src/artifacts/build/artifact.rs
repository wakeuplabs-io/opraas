
pub trait BuildArtifact {
   fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
   fn build(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
   fn needs_push(&self, cfg: &crate::config::Config) -> bool;
   fn push(&self, cfg: &crate::config::Config, repository: &str) -> Result<(), Box<dyn std::error::Error>>;
}
