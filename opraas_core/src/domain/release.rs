use super::{Artifact, ArtifactFactory, ArtifactKind, Project};
use crate::config::CoreConfig;
use mockall::automock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub artifact_name: String,
    pub artifact_tag: String,
    pub registry_url: String,
}

pub struct ReleaseFactory {
    artifacts_factory: ArtifactFactory,
}

#[automock]
pub trait TReleaseRepository {
    fn create_for_artifact(
        &self,
        artifact: &Artifact,
        release_name: &str,
        registry_url: &str,
    ) -> Result<Release, Box<dyn std::error::Error>>;
    fn pull(&self, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
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

impl ReleaseFactory {
    pub fn new(project: &Project, config: &CoreConfig) -> Self {
        Self {
            artifacts_factory: ArtifactFactory::new(project, config),
        }
    }

    pub fn get(&self, kind: ArtifactKind, release_name: &str, registry_url: &str) -> Arc<Release> {
        Arc::new(Release::from_artifact(
            &self.artifacts_factory.get(kind),
            release_name,
            registry_url,
        ))
    }

    pub fn get_all(&self, release_name: &str, registry_url: &str) -> Vec<Arc<Release>> {
        self.artifacts_factory
            .get_all()
            .iter()
            .map(|artifact| {
                Arc::new(Release::from_artifact(
                    &artifact,
                    release_name,
                    registry_url,
                ))
            })
            .collect()
    }
}
