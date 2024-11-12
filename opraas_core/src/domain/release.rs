use super::{Artifact};


pub struct  Release {
    name: String,
    repository: String,
}

impl Release {
    pub fn new(name: String, repository: String) -> Self {
        Self {
            name,
            repository,
        }
    }

    pub fn build_artifact_uri(&self, artifact_name: String) -> String {
        format!("{}/{}:{}", self.repository, artifact_name, self.name)
    }
}

pub trait TArtifactReleaseRepository {
    fn exists(&self, artifact: &Artifact) -> bool;
    fn create(&self, artifact: &Artifact, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
    fn pull(&self, artifact: &Artifact, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
}