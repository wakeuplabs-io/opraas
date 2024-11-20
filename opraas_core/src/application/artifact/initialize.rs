use crate::domain;
use crate::domain::artifact::Artifact;
use crate::infra::repositories::artifact_source::GitArtifactSourceRepository;

pub struct ArtifactInitializer {
    source_repository: Box<dyn domain::artifact::TArtifactSourceRepository>,
}

pub trait TArtifactInitializerService {
    fn initialize(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations =================================================

impl ArtifactInitializer {
    pub fn new() -> Self {
        Self {
            source_repository: Box::new(GitArtifactSourceRepository::new()),
        }
    }
}

impl TArtifactInitializerService for ArtifactInitializer {
    fn initialize(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        if self.source_repository.exists(artifact) {
            return Ok(());
        }

        self.source_repository.pull(artifact)?;

        Ok(())
    }
}
