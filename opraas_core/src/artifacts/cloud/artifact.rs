pub trait CloudArtifact {
    fn deploy(&self, cfg: &crate::config::Config);
    fn remove(&self, cfg: &crate::config::Config);
    fn inspect(&self, cfg: &crate::config::Config);
    fn monitor(&self, cfg: &crate::config::Config);
}
