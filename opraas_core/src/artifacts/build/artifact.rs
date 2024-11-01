use crate::progress::ProgressTracker;


pub trait BuildArtifact {
   fn setup(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>>;
   fn build(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>>;
}
