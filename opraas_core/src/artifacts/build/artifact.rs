
pub trait BuildArtifact {
   fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
   fn build(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
}
