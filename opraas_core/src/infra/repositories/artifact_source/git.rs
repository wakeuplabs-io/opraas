use crate::{
    domain::{self, artifact::Artifact},
    git,
};

pub struct GitArtifactSourceRepository;

impl GitArtifactSourceRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::artifact::TArtifactSourceRepository for GitArtifactSourceRepository {
    fn pull(
        &self,
        artifact: &Artifact,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (release_url, release_tag) = artifact.source_info();

        // TODO: refactor download release to take better paths
        git::download_release(release_url, release_tag, artifact.context().as_path().to_str().unwrap())?;

        // download dockerfile for infra

        Ok(())
    }

    fn exists(&self, artifact: &Artifact) -> bool {
        artifact.context().exists()
    }
}
