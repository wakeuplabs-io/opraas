use crate::config::{artifacts::ArtifactConfig, CoreConfig};
use std::{path::PathBuf, sync::Arc};

use super::project::Project;

pub struct Artifact {
    pub source: PathBuf,
    pub release_url: String,
    pub release_tag: String,
    pub image_tag: String,
}

#[derive(Debug)]
pub enum ArtifactKind {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
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

impl Artifact {
    pub fn new(source: &PathBuf, config: &ArtifactConfig) -> Self {
        Self {
            source: source.to_path_buf(),
            release_url: config.release_url.clone(),
            release_tag: config.release_tag.clone(),
            image_tag: config.image_tag.clone(),
        }
    }
}

pub struct ArtifactFactory;

impl ArtifactFactory {
    pub fn create_artifacts(
        target: ArtifactKind,
        project: &Project,
        cfg: &CoreConfig,
    ) -> Vec<(&'static str, Arc<Artifact>)> {
        let mut artifacts = Vec::new();

        match target {
            ArtifactKind::Batcher => {
                artifacts.push((
                    "Batcher",
                    Arc::new(Artifact::new(&project.src.batcher, &cfg.artifacts.batcher)),
                ));
            }
            ArtifactKind::Node => {
                artifacts.push((
                    "Node",
                    Arc::new(Artifact::new(&project.src.node, &cfg.artifacts.node)),
                ));
            }
            // ... Additional targets as in previous example
            ArtifactKind::All => {
                artifacts.push((
                    "Batcher",
                    Arc::new(Artifact::new(&project.src.batcher, &cfg.artifacts.batcher)),
                ));
                artifacts.push((
                    "Node",
                    Arc::new(Artifact::new(&project.src.node, &cfg.artifacts.node)),
                ));
                // ... Add the other targets here
            }
            _ => {}
        }

        artifacts
    }
}
