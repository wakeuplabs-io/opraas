use crate::domain;
use crate::infra::repositories::{
    artifact_source::GitArtifactSourceRepository, project::InMemoryProjectRepository,
};

pub trait TArtifactInitializerService {
    fn initialize(
        &self,
        artifact: &domain::artifact::Artifact,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

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

impl TArtifactInitializerService for ArtifactInitializer {
    fn initialize(
        &self,
        artifact: &domain::artifact::Artifact,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.project_repository.exists(&artifact.source) {
            return Ok(());
        }

        self.source_repository.pull(
            &artifact.release_url,
            &artifact.release_tag,
            artifact.source.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }
}
