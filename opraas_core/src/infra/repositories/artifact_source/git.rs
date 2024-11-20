use crate::{
    domain::{self, artifact::Artifact},
    git,
};

pub struct GitArtifactSourceRepository;

const OP_RUAAS_REPO: &str = "wakeuplabs-io/op-ruaas";
const OP_RUAAS_VERSION: &str = "v0.0.2";

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
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile", // TODO: replace once proper images are available
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            },
            Artifact::Contracts(..) => {
                git::download_release_asset(
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile", // TODO: replace once proper images are available
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            },
            Artifact::Explorer(..) => {
                git::download_release_asset(
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile", // TODO: replace once proper images are available
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            },
            Artifact::Proposer(..) => {
                git::download_release_asset(
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile", // TODO: replace once proper images are available
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            },
            Artifact::Geth(..) => {
                git::download_release_asset(
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile", // TODO: replace once proper images are available
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            },
            Artifact::Node(..) => {
                git::download_release_asset(
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile", // TODO: replace once proper images are available
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
