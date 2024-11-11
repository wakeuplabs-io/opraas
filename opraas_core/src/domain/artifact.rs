use std::path::PathBuf;
use crate::config::artifacts::ArtifactConfig;

pub struct Artifact<'a> {
    pub source: PathBuf,
    pub release_url: &'a str,
    pub release_tag: &'a str,
    pub image_tag: &'a str,
}

impl<'a> Artifact<'a> {
    pub fn new(source: &PathBuf, config: &'a ArtifactConfig) -> Self {
        Self {
            source: source.to_path_buf(),
            release_url: &config.release_url,
            release_tag: &config.release_tag,
            image_tag: &config.image_tag,
        }
    }
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
