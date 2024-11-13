use super::Artifact;

pub struct Release {
    pub artifact_name: String,
    pub artifact_tag: String,
    pub registry_url: String,
}

pub trait TReleaseRepository {
    fn create_for_artifact(
        &self,
        artifact: &Artifact,
        release_name: &str, 
        registry_url: &str
    ) -> Result<Release, Box<dyn std::error::Error>>;
    fn pull(&self, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations =============================================

impl Release {
    pub fn new(artifact_name: String, artifact_tag: String, registry_url: String) -> Self {
        // TODO: validations
        Self {
            artifact_name,
            artifact_tag,
            registry_url,
        }
    }

    pub fn from_artifact(artifact: &Artifact, release_name: &str, registry_url: &str) -> Self {
        // TODO: validations
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
