use crate::{git, progress::ProgressTracker};

pub struct ExplorerBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl ExplorerBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for ExplorerBuildArtifact {

    fn download(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        self.downloader.download_release(
            &cfg.core.sources.explorer.base_url,
            &cfg.core.sources.explorer.release_tag,
            &cfg.tree.src.explorer.as_path().to_str().unwrap(),
            progress
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config, _progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


