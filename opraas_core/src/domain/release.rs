use super::{Artifact};


pub struct  Release {
    artifact_name: String,
    artifact_tag: String,
    repository: String,
}

pub trait TReleaseRepository {
    fn create_for_artifact(&self, artifact: &Artifact, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
    fn pull(&self, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations =============================================

impl Release {
    pub fn new(artifact_name: String, artifact_tag: String, repository: String) -> Self {
        Self {
            artifact_name,
            artifact_tag,
            repository,
        }
    }

    pub fn build_artifact_uri(&self, artifact_name: String) -> String {
        format!("{}/{}:{}", self.repository, artifact_name, self.name)
    }
}

