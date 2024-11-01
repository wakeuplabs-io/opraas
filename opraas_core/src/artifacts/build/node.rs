use crate::{git, progress::ProgressTracker};

pub struct NodeBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl NodeBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for NodeBuildArtifact {

    fn setup(&self, cfg: &crate::config::Config, progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        if cfg.tree.src.node.exists() {
            return Ok(());
        }
        
        self.downloader.download_release(
            &cfg.core.sources.node.release_url,
            &cfg.core.sources.node.release_tag,
            &cfg.tree.src.node.as_path().to_str().unwrap(),
            progress
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config, _progress: &dyn ProgressTracker) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


