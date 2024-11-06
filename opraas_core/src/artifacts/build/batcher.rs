use std::process::Command;

use crate::{filesystem, git, system};

pub struct BatcherBuildArtifact {
    system: Box<dyn system::TSystem>,
    filesystem: Box<dyn filesystem::Filesystem>,
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl BatcherBuildArtifact {
    pub fn new() -> Self {
        Self {
            system: Box::new(system::System::new()),
            downloader: Box::new(git::Git::new()),
            filesystem: Box::new(filesystem::Fs::new()),
        }
    }
}

impl crate::artifacts::build::BuildArtifact for BatcherBuildArtifact {
    fn setup(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if self.filesystem.exists(&cfg.tree.src.batcher) {
            return Ok(());
        }

        self.downloader.download_release(
            &cfg.core.artifacts.batcher.release_url,
            &cfg.core.artifacts.batcher.release_tag,
            &cfg.tree.src.batcher.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }

    fn build(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if !self.filesystem.exists(&cfg.tree.src.batcher) {
            return Err("Batcher src is not available".into());
        }

        if !self.filesystem.exists(&cfg.tree.infra.docker.batcher) {
            return Err(format!("Batcher dockerfile is not available at {:?}", &cfg.tree.infra.docker.batcher.display()).into());
        }

        // build batcher
        // let mut command = Command::new("docker");
        // command
        //     .current_dir(&cfg.tree.src.batcher)
        //     .arg("build")
        //     .arg("-f")
        //     .arg(&cfg.tree.infra.docker.batcher)
        //     .arg("-t")
        //     .arg("batcher/tag")
        //     .arg(&cfg.tree.src.batcher);

        // self.system.execute_command(&mut command)?;

        Ok(())
    }

    fn needs_push(&self, cfg: &crate::config::Config) -> bool {
        true
    }

    fn push(&self, cfg: &crate::config::Config, repository: &str) -> Result<(), Box<dyn std::error::Error>> {
        // wait 1 min
        std::thread::sleep(std::time::Duration::from_secs(60));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::build::artifact::BuildArtifact;
    use crate::config::Config;
    use mockall::predicate;

    #[test]
    fn test_setup_downloads_release_if_not_exists() {
        let config = Config {
            tree: crate::config::TreeConfig::new_from_root(std::env::current_dir().unwrap()),
            core: crate::config::CoreConfig::new_from_null(),
        };

        let mut mock_filesystem = filesystem::MockFilesystem::new();
        mock_filesystem
            .expect_exists()
            .with( predicate::eq(config.tree.src.batcher.clone()))
            .times(1) // Expect 1 call to `exists`
            .returning(|_| false); // Return false to indicate that the file does not exist

        let mut mock_downloader = git::MockGitReleaseDownloader::new();
        mock_downloader
            .expect_download_release()
            .times(1) // Expect 1 call to `download_release`
            .returning(|_, _, _| Ok(())); // Return Ok to indicate successful download

        let mock_system = system::MockTSystem::new();

        let batcher_artifact = BatcherBuildArtifact {
            system: Box::new(mock_system),
            downloader: Box::new(mock_downloader),
            filesystem: Box::new(mock_filesystem),
        };

        // act
        let result = batcher_artifact.setup(&config);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_does_not_download_if_exists() {
        let config = Config {
            tree: crate::config::TreeConfig::new_from_root(std::env::current_dir().unwrap()),
            core: crate::config::CoreConfig::new_from_null(),
        };

        let mut mock_filesystem = filesystem::MockFilesystem::new();
        mock_filesystem
            .expect_exists()
            .with( predicate::eq(config.tree.src.batcher.clone()))
            .times(1) // Expect 1 call to `exists`
            .returning(|_| true); // Return true to indicate that the file exists

        let mut mock_downloader = git::MockGitReleaseDownloader::new();
        mock_downloader.expect_download_release().times(0); // Expect 0 calls to `download_release`

        let mock_system = system::MockTSystem::new();

        let batcher_artifact = BatcherBuildArtifact {
            system: Box::new(mock_system),
            downloader: Box::new(mock_downloader),
            filesystem: Box::new(mock_filesystem),
        };

        // act
        let result = batcher_artifact.setup(&config);

        // assert
        assert!(result.is_ok());
    }
}
