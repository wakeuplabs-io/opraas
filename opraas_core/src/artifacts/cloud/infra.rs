use crate::{filesystem, git};

pub struct InfraCloudArtifact {
    filesystem: Box<dyn filesystem::Filesystem>,
    downloader: Box<dyn git::GitReleaseDownloader>,
}

impl InfraCloudArtifact {
    pub fn new() -> Self {
        Self {
            filesystem: Box::new(filesystem::Fs::new()),
            downloader: Box::new(git::Git::new()),
        }
    }
}

impl crate::artifacts::initializable::Initializable for InfraCloudArtifact {
    fn initialize(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if self.filesystem.exists(&cfg.tree.infra.root) {
            return Ok(());
        }

        self.downloader.download_release_zipped_asset(
            &cfg.core.artifacts.infra.release_url,
            &cfg.core.artifacts.infra.release_tag,
            "infra",
            &cfg.tree.infra.root.as_path().to_str().unwrap(),
        )?;

        Ok(())
    }
}
