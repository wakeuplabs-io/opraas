use crate::{
    domain::{self, artifact::Artifact, Release},
    infra,
};

pub struct ArtifactReleaserService {
    release_repository: Box<dyn domain::release::TReleaseRepository>,
}

pub trait TArtifactReleaserService {
    fn release(
        &self,
        artifact: &Artifact,
        release_name: &str,
        registry_url: &str,
    ) -> Result<Release, Box<dyn std::error::Error>>;
}

// implementations ======================================================

impl ArtifactReleaserService {
    pub fn new() -> Self {
        Self {
            release_repository: Box::new(infra::repositories::release::DockerReleaseRepository::new()),
        }
    }
}

impl TArtifactReleaserService for ArtifactReleaserService {
    fn release(
        &self,
        artifact: &Artifact,
        release_name: &str,
        registry_url: &str,
    ) -> Result<Release, Box<dyn std::error::Error>> {
        self.release_repository
            .create_for_artifact(&artifact, release_name, registry_url)
    }
}

#[cfg(test)]
mod test {
    use super::{ArtifactReleaserService, TArtifactReleaserService};
    use crate::domain::{Artifact, ArtifactData, MockTReleaseRepository, Release};
    use std::path::PathBuf;

    #[test]
    fn creates_release_for_artifact() {
        let mut mock_release_repository = MockTReleaseRepository::new();

        let artifact = Artifact::Batcher(ArtifactData {
            name: "mock".to_string(),
            context: PathBuf::new(),
            dockerfile: PathBuf::new(),
            source_tag: "v0.0.1".to_string(),
            source_url: "http://github.com".to_string(),
        });

        mock_release_repository
            .expect_create_for_artifact()
            .returning(|_, _, _| {
                Ok(Release {
                    artifact_name: "artifact_name".to_string(),
                    artifact_tag: "artifact_tag".to_string(),
                    registry_url: "registry_url".to_string(),
                })
            });

        let service = ArtifactReleaserService {
            release_repository: Box::new(mock_release_repository),
        };

        let result = service.release(&artifact, "release_name", "wakeuplabs");
        assert!(result.is_ok());
    }
}
