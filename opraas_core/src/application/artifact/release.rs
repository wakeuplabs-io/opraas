use crate::{domain::{self, artifact::Artifact, Release}, infra};

pub struct ArtifactReleaserService {
    artifact_release_repository: Box<dyn domain::release::TArtifactReleaseRepository>,
}

pub trait TArtifactReleaserService {
    fn release(&self, artifact: &Artifact, release: &Release) -> Result<(), Box<dyn std::error::Error>>;
}

impl ArtifactReleaserService {
    pub fn new() -> Self {
        Self {
            artifact_release_repository: Box::new(infra::repositories::artifact::DockerArtifactReleaser::new()),
        }
    }
}

impl TArtifactReleaserService for ArtifactReleaserService {
    fn release(&self, artifact: &Artifact, release: &Release) -> Result<(), Box<dyn std::error::Error>> {
        if !self.artifact_release_repository.exists(artifact) {
            return Err(format!("Image {} for artifact not found", artifact.name()).into())
        }

        self.artifact_release_repository.create(artifact, release)?;
        
        Ok(())
    }
    
}