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

        git::download_release(
            source_repo,
            source_tag,
            artifact.context().as_path().to_str().unwrap(),
        )?;

        // download dockerfile for infra
        match artifact {
            // TODO: add more
            Artifact::Batcher(..) => {
                git::download_release_asset(
                    OP_RUAAS_REPO,
                    OP_RUAAS_VERSION,
                    "infra/docker/Node.dockerfile",
                    artifact.dockerfile().as_path().to_str().unwrap(),
                )?;
            }
            _ => panic!("not implemented"),
        };

        Ok(())
    }

    fn exists(&self, artifact: &Artifact) -> bool {
        artifact.context().exists()
    }
}
