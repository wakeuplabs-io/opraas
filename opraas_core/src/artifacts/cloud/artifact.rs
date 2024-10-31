use crate::progress::ProgressTracker;

pub trait CloudArtifact {
    fn deploy<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T);
    fn remove<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T);
    fn inspect<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T);
    fn monitor<T: ProgressTracker>(&self, cfg: &crate::config::Config, progress: &T);
}
