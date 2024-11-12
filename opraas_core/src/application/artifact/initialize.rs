use crate::domain;
use crate::domain::artifact::Artifact;
use crate::infra::repositories::{
    artifact_source::GitArtifactSourceRepository, project::InMemoryProjectRepository,
};

pub struct ArtifactInitializer {
    project_repository: Box<dyn domain::project::TProjectRepository>,
    source_repository: Box<dyn domain::artifact::TArtifactSourceRepository>,
}

impl ArtifactInitializer {
    pub fn new() -> Self {
        Self {
            project_repository: Box::new(InMemoryProjectRepository::new()),
            source_repository: Box::new(GitArtifactSourceRepository::new()),
        }
    }
}

pub trait TArtifactInitializerService {
    fn initialize(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

impl TArtifactInitializerService for ArtifactInitializer {
    fn initialize(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        if self.project_repository.exists(artifact.context()) {
            return Ok(());
        }

        self.source_repository.pull(artifact)?;

        Ok(())
    }
}
