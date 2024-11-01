use crate::progress::ProgressTracker;
use crate::config::Config;

pub trait CloudArtifact {
    fn deploy(&self, cfg: &Config, progress: &dyn ProgressTracker);
    fn remove(&self, cfg: &Config, progress: &dyn ProgressTracker);
    fn inspect(&self, cfg: &Config, progress: &dyn ProgressTracker);
    fn monitor(&self, cfg: &Config, progress: &dyn ProgressTracker);
}
