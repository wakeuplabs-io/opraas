use crate::{domain::{self, artifact::Artifact}, infra};

pub struct ArtifactReleaserService {
    artifact_release_repository: Box<dyn domain::artifact::TArtifactReleaseRepository>,
}

pub trait TArtifactReleaserService {
    fn release(&self, artifact: &Artifact, name: &str, repository: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl ArtifactReleaserService {
    pub fn new() -> Self {
        Self {
            artifact_release_repository: Box::new(infra::repositories::artifact_release::DockerArtifactReleaser::new()),
        }
    }
}

impl TArtifactReleaserService for ArtifactReleaserService {
    fn release(&self, artifact: &Artifact, name: &str, repository: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.artifact_release_repository.exists(artifact) {
            return Ok(());
        }

        self.artifact_release_repository.push(artifact)?;
        
        Ok(())
    }
    
}