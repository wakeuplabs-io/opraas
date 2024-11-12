use crate::domain;

pub struct  DockerArtifactReleaser;

impl DockerArtifactReleaser {
    pub fn new() -> Self {
        Self
    }
}

impl domain::TArtifactReleaseRepository for DockerArtifactReleaser {
    fn exists(&self, artifact: &domain::Artifact) -> bool {
        true
    }

    fn pull(&self, artifact: &domain::Artifact) -> Result<(), Box<dyn std::error::Error>> {
        println!("Pulling artifact: {}", artifact.context().display());
        Ok(())
    }

    fn push(&self, artifact: &domain::Artifact) -> Result<(), Box<dyn std::error::Error>> {
        println!("Pushing artifact: {}", artifact.context().display());
        Ok(())
    }
}