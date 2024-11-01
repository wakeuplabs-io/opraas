use crate::{git, progress::ProgressTracker};

pub struct ProposerBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl ProposerBuildArtifact {
    pub fn new() -> Self {
        Self {
            downloader: Box::new(git::Git::new()),
        }
    }
}

impl crate::artifacts::build::BuildArtifact for ProposerBuildArtifact {
    fn setup(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        if cfg.tree.src.proposer.exists() {
            return Ok(());
        }

        self.downloader.download_release(
            &cfg.core.sources.proposer.release_url,
            &cfg.core.sources.proposer.release_tag,
            &cfg.tree.src.proposer.as_path().to_str().unwrap(),
            progress,
        )?;

        Ok(())
    }

    fn build(
        &self,
        _cfg: &crate::config::Config,
        _progress: &dyn ProgressTracker,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
