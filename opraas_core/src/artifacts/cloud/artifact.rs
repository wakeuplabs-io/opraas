use crate::config::Config;

pub trait CloudArtifact {
    fn deploy(&self, cfg: &Config);
    fn remove(&self, cfg: &Config);
    fn inspect(&self, cfg: &Config);
    fn monitor(&self, cfg: &Config);
}
