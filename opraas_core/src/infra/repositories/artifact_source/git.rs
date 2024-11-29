use crate::{
    config::artifacts::{INFRA_SOURCE_REPO, INFRA_SOURCE_REPO_VERSION}, domain::{self, artifact::Artifact}, git
};

pub struct GitArtifactSourceRepository;



impl GitArtifactSourceRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::artifact::TArtifactSourceRepository for GitArtifactSourceRepository {
    fn pull(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
        let (source_repo, source_tag) = artifact.source_info();

        git::clone(
            source_repo,
            source_tag,
            artifact.context().as_path().to_str().unwrap(),
        )?;

        // download dockerfile for infra
        match artifact {
            Artifact::Batcher(..) => {
                git::download_release_asset(
                    INFRA_SOURCE_REPO,
                    INFRA_SOURCE_REPO_VERSION,
                    "infra/docker/batcher.dockerfile",
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            }
            Artifact::Contracts(..) => {
                git::download_release_asset(
                    INFRA_SOURCE_REPO,
                    INFRA_SOURCE_REPO_VERSION,
                    "infra/docker/contracts.dockerfile",
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            }
            Artifact::Proposer(..) => {
                git::download_release_asset(
                    INFRA_SOURCE_REPO,
                    INFRA_SOURCE_REPO_VERSION,
                    "infra/docker/proposer.dockerfile",
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            }
            Artifact::Geth(..) => {
                git::download_release_asset(
                    INFRA_SOURCE_REPO,
                    INFRA_SOURCE_REPO_VERSION,
                    "infra/docker/geth.dockerfile",
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            }
            Artifact::Node(..) => {
                git::download_release_asset(
                    INFRA_SOURCE_REPO,
                    INFRA_SOURCE_REPO_VERSION,
                    "infra/docker/node.dockerfile",
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            }
        };

        Ok(())
    }

    fn exists(&self, artifact: &Artifact) -> bool {
        artifact.context().exists()
    }
}
