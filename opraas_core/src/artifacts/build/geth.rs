use crate::git;

pub struct GethBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl GethBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for GethBuildArtifact {

    fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if cfg.tree.src.geth.exists() {
            return Ok(());
        }

        self.downloader.download_release(
            &cfg.core.sources.geth.release_url,
            &cfg.core.sources.geth.release_tag,
            &cfg.tree.src.geth.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


