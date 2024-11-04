use crate::git;

pub struct ExplorerBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl ExplorerBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for ExplorerBuildArtifact {

    fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if cfg.tree.src.explorer.exists() {
            return Ok(());
        }
        
        self.downloader.download_release(
            &cfg.core.sources.explorer.release_url,
            &cfg.core.sources.explorer.release_tag,
            &cfg.tree.src.explorer.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


