use crate::{domain, git};

pub struct GitArtifactSource;

impl domain::artifact::TArtifactSourceRepository for GitArtifactSource {
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
