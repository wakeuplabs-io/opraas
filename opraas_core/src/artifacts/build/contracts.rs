use crate::git;

pub struct ContractsBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl ContractsBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for ContractsBuildArtifact {

    fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if cfg.tree.src.contracts.exists() {
            return Ok(());
        }

        self.downloader.download_release(
            &cfg.core.sources.contracts.release_url,
            &cfg.core.sources.contracts.release_tag,
            &cfg.tree.src.contracts.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}


