use std::sync::Arc;
use opraas_core::{config::CoreConfig, domain::{Artifact, ArtifactKind, Project}};

pub enum ArtifactFactoryTarget {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

pub fn create_artifacts(target: ArtifactFactoryTarget, project: &Project, config: &CoreConfig) -> Vec<(&'static str, Arc<Artifact>)> {
    let mut artifacts: Vec<(&'static str, Arc<Artifact>)> = vec![];

    match target {
        ArtifactFactoryTarget::Batcher => {
            let artifact = Artifact::new(
                ArtifactKind::Batcher,
                &project.src.batcher,
                &project.infra.docker.batcher,
                &config.artifacts.batcher,
            );
            artifacts.push(("batcher", Arc::new(artifact)));
        }
        ArtifactFactoryTarget::Node => {
            let artifact = Artifact::new(
                ArtifactKind::Node,
                &project.src.node,
                &project.infra.docker.node,
                &config.artifacts.node,
            );
            artifacts.push(("node", Arc::new(artifact)));
        }
        ArtifactFactoryTarget::Contracts => {
            let artifact = Artifact::new(
                ArtifactKind::Contracts,
                &project.src.contracts,
                &project.infra.docker.contracts,
                &config.artifacts.contracts,
            );
            artifacts.push(("contracts", Arc::new(artifact)));
        }
        ArtifactFactoryTarget::Explorer => {
            let artifact = Artifact::new(
                ArtifactKind::Explorer,
                &project.src.explorer,
                &project.infra.docker.explorer,
                &config.artifacts.explorer,
            );
            artifacts.push(("explorer", Arc::new(artifact)));
        }
        ArtifactFactoryTarget::Proposer => {
            let artifact = Artifact::new(
                ArtifactKind::Proposer,
                &project.src.proposer,
                &project.infra.docker.proposer,
                &config.artifacts.proposer,
            );
            artifacts.push(("proposer", Arc::new(artifact)));
        }
        ArtifactFactoryTarget::Geth => {
            let artifact = Artifact::new(
                ArtifactKind::Geth,
                &project.src.geth,
                &project.infra.docker.geth,
                &config.artifacts.geth,
            );
            artifacts.push(("geth", Arc::new(artifact)));
        }
        ArtifactFactoryTarget::All => {
            let artifact = Artifact::new(
                ArtifactKind::Batcher,
                &project.src.batcher,
                &project.infra.docker.batcher,
                &config.artifacts.batcher,
            );
            artifacts.push(("batcher", Arc::new(artifact)));

            let artifact = Artifact::new(
                ArtifactKind::Node,
                &project.src.contracts,
                &project.infra.docker.contracts,
                &config.artifacts.contracts,
            );
            artifacts.push(("node", Arc::new(artifact)));

            let artifact = Artifact::new(
                ArtifactKind::Contracts,
                &project.src.contracts,
                &project.infra.docker.contracts,
                &config.artifacts.contracts,
            );
            artifacts.push(("contracts", Arc::new(artifact)));

            let artifact = Artifact::new(
                ArtifactKind::Explorer,
                &project.src.explorer,
                &project.infra.docker.explorer,
                &config.artifacts.explorer,
            );
            artifacts.push(("explorer", Arc::new(artifact)));

            let artifact = Artifact::new(
                ArtifactKind::Proposer,
                &project.src.proposer,
                &project.infra.docker.proposer,
                &config.artifacts.proposer,
            );
            artifacts.push(("proposer", Arc::new(artifact)));

            let artifact = Artifact::new(
                ArtifactKind::Geth,
                &project.src.geth,
                &project.infra.docker.geth,
                &config.artifacts.geth,
            );
            artifacts.push(("geth", Arc::new(artifact)));
        }
    }

    artifacts
}
