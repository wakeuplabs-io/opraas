use crate::config::Config;

pub trait CloudArtifact {
    fn setup(&self, cfg: &Config) -> Result<(), Box<dyn std::error::Error>>;
    fn deploy(&self, cfg: &Config, name: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn remove(&self, cfg: &Config, name: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn inspect(&self, cfg: &Config, name: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn monitor(&self, cfg: &Config, name: &str) -> Result<(), Box<dyn std::error::Error>>;
}
