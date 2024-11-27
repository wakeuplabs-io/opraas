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
