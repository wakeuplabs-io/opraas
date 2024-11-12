use crate::{domain::{self, artifact::Artifact}, infra::{self, artifact_builder::TArtifactBuilder}};

pub struct  ArtifactBuilderService {
    artifact_builder: Box<dyn TArtifactBuilder>,
    artifact_source_repository: Box<dyn domain::artifact::TArtifactSourceRepository>,
    artifact_release_repository: Box<dyn domain::artifact::TArtifactReleaseRepository>,
}

impl ArtifactBuilderService {
    pub fn new() -> Self {
        Self {
            artifact_builder: Box::new(infra::artifact_builder::DockerArtifactBuilder::new()),
            artifact_source_repository: Box::new(infra::repositories::artifact_source::GitArtifactSourceRepository::new()),
            artifact_release_repository: Box::new(infra::repositories::artifact_release::DockerArtifactReleaser::new()),
        }
    }
}

pub trait TArtifactBuilderService {
    fn build(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
    fn release(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

impl TArtifactBuilderService for ArtifactBuilderService {
    fn build(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        if !self.artifact_source_repository.exists(artifact) {
            self.artifact_source_repository.pull(artifact)?;
        }

        self.artifact_builder.build_artifact(artifact)?;

        Ok(())
    }

    fn release(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        if self.artifact_release_repository.exists(artifact) {
            return Ok(());
        }

        self.artifact_release_repository.push(artifact)?;
        
        Ok(())
    }
    
}