use crate::{domain, git};

pub struct GitArtifactSourceRepository;

impl GitArtifactSourceRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::artifact::TArtifactSourceRepository for GitArtifactSourceRepository {
    fn pull(
        &self,
        release_url: &str,
        release_tag: &str,
        destination: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        git::download_release(release_url, release_tag, destination)?;

        Ok(())
    }
}
