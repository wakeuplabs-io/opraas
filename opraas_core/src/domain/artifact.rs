use crate::config::Config;
use std::path::PathBuf;

pub struct Artifact<'a> {
    pub source: PathBuf,
    pub release_url: &'a str,
    pub release_tag: &'a str,
    pub image_tag: &'a str,
}

impl<'a> Artifact<'a> {
    pub fn new(source: PathBuf) -> Self {
        Self {
            source,
            release_url: "",
            release_tag: "",
            image_tag: "",
        }
    }
}

pub trait TArtifactInitializerService {
    fn initialize(
        &self,
        cfg: &Config,
        artifact: &Artifact,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait TArtifactBuilderService {
    fn build(&self, cfg: &Config, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
    fn release(&self, cfg: &Config, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait TArtifactDeployerService {
    fn deploy(&self, cfg: &Config, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait TArtifactSourceRepository {
    fn pull(
        &self,
        release_url: &str,
        release_tag: &str,
        destination: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait TArtifactReleaseRepository {
    fn pull();
    fn push();
}
