use crate::{git, progress::ProgressTracker};

pub struct BatcherBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl BatcherBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for BatcherBuildArtifact {

    fn download(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        self.downloader.download_release(
            &cfg.core.sources.batcher.base_url,
            &cfg.core.sources.batcher.release_tag,
            &cfg.tree.src.batcher.as_path().to_str().unwrap(),
            progress
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config, _progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


