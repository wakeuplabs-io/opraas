use std::{collections::HashMap, path::Path};

use super::Artifact;
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub artifact_name: String,
    pub artifact_tag: String,
    pub registry_url: String,
}

#[automock]
pub trait TReleaseRepository: Send + Sync {
    fn create_for_artifact(
        &self,
        artifact: &Artifact,
        release_name: &str,
        registry_url: &str,
    ) -> Result<Release, Box<dyn std::error::Error>>;
    fn pull(&self, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait TReleaseRunner: Send + Sync {
    fn run(
        &self,
        release: &Release,
        volume: &Path,
        env: HashMap<&str, String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations =============================================

impl Release {
    pub fn new(artifact_name: String, artifact_tag: String, registry_url: String) -> Self {
        if artifact_name.is_empty() {
            panic!("Artifact name can't be empty");
        }
        if artifact_tag.is_empty() {
            panic!("Artifact tag can't be empty");
        }
        if registry_url.is_empty() {
            panic!("Registry url can't be empty");
        }

        Self {
            artifact_name,
            artifact_tag,
            registry_url,
        }
    }

    pub fn from_artifact(artifact: &Artifact, release_name: &str, registry_url: &str) -> Self {
        if artifact.name().is_empty() {
            panic!("Artifact name can't be empty");
        }
        if release_name.is_empty() {
            panic!("Artifact tag can't be empty");
        }
        if registry_url.is_empty() {
            panic!("Registry url can't be empty");
        }

        Self {
            artifact_name: artifact.name().to_string(),
            artifact_tag: release_name.to_string(),
            registry_url: registry_url.to_string(),
        }
    }

    pub fn uri(&self) -> String {
        format!(
            "{}/{}:{}",
            self.registry_url, self.artifact_name, self.artifact_tag
        )
    }
}
