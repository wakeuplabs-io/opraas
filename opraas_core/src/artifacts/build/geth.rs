use crate::{git, progress::ProgressTracker};

pub struct GethBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl GethBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for GethBuildArtifact {

    fn download(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        self.downloader.download_release(
            &cfg.core.sources.geth.base_url,
            &cfg.core.sources.geth.release_tag,
            &cfg.tree.src.geth.as_path().to_str().unwrap(),
            progress
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config, _progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


