use crate::domain;

pub struct  DockerArtifactReleaser;

impl DockerArtifactReleaser {
    pub fn new() -> Self {
        Self
    }
}

impl domain::TArtifactReleaseRepository for DockerArtifactReleaser {
    fn exists(&self, artifact: &domain::Artifact) -> bool {
        todo!()
    }

    fn pull(&self, artifact: &domain::Artifact) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn push(&self, artifact: &domain::Artifact) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}