use crate::git;

pub struct BatcherBuildArtifact {
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl BatcherBuildArtifact {
    pub fn new() -> Self {
        Self { downloader: Box::new(git::Git::new()) }
    }
}

impl crate::artifacts::build::BuildArtifact for BatcherBuildArtifact {

    fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if cfg.tree.src.batcher.exists() {
            return Ok(());
        }

        self.downloader.download_release(
            &cfg.core.sources.batcher.release_url,
            &cfg.core.sources.batcher.release_tag,
            &cfg.tree.src.batcher.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }

    fn build(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config; // Import necessary modules and traits
    use crate::artifacts::build::artifact::BuildArtifact;
    

    #[test]
    fn test_setup_downloads_release_if_not_exists() {
        std::env::set_var("L1_RPC_URL", "mocked_value");

        let mut mock_downloader = git::MockGitReleaseDownloader::new();
        mock_downloader.expect_download_release().times(1).returning(|_, _, _| Ok(()));
        
        let config = Config {
            tree: crate::config::TreeConfig::new_from_root(std::env::current_dir().unwrap()),
            core: crate::config::CoreConfig::new_from_null(),
        };

        let batcher_artifact = BatcherBuildArtifact {
            downloader: Box::new(mock_downloader),
        };

        // Act
        let result = batcher_artifact.setup(&config);

        // Assert
        assert!(result.is_ok());
    }

    // #[test]
    // fn test_setup_does_not_download_if_exists() {
       
    // }


}
