use crate::{
    domain::{self, artifact::Artifact},
    infra,
};

pub struct ArtifactBuilderService {
    artifact_repository: Box<dyn domain::artifact::TArtifactRepository>,
    artifact_source_repository: Box<dyn domain::artifact::TArtifactSourceRepository>,
}

impl ArtifactBuilderService {
    pub fn new() -> Self {
        Self {
            artifact_repository: Box::new(
                infra::repositories::artifact::DockerArtifactRepository::new(),
            ),
            artifact_source_repository: Box::new(
                infra::repositories::artifact_source::GitArtifactSourceRepository::new(),
            ),
        }
    }
}

pub trait TArtifactBuilderService {
    fn build(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

impl TArtifactBuilderService for ArtifactBuilderService {
    fn build(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        if !self.artifact_source_repository.exists(artifact) {
            self.artifact_source_repository.pull(artifact)?;
        }

        self.artifact_repository.create(artifact)?;

        Ok(())
    }
}
