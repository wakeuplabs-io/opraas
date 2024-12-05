use mockall::automock;
use crate::config::{artifacts::ArtifactConfig, CoreConfig};
use std::{collections::HashMap, fmt, path::PathBuf, sync::Arc};

use super::Project;

#[derive(Debug)]
pub struct ArtifactData {
    pub name: String,
    pub context: PathBuf,
    pub dockerfile: PathBuf,
    pub source_url: String,
    pub source_tag: String,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ArtifactKind {
    Batcher,
    Node,
    Contracts,
    Proposer,
    Geth,
}

#[derive(Debug)]
pub enum Artifact {
    Batcher(ArtifactData),
    Node(ArtifactData),
    Contracts(ArtifactData),
    Proposer(ArtifactData),
    Geth(ArtifactData),
}

impl fmt::Display for Artifact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Artifact::Batcher(_data) => write!(f, "Batcher"),
            Artifact::Node(_data) => write!(f, "Node"),
            Artifact::Contracts(_data) => write!(f, "Contracts"),
            Artifact::Proposer(_data) => write!(f, "Proposer"),
            Artifact::Geth(_data) => write!(f, "Geth"),
        }
    }
}

pub struct ArtifactFactory {
    artifacts: HashMap<ArtifactKind, Arc<Artifact>>,
}

#[automock]
pub trait TArtifactSourceRepository {
    fn pull(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
    fn exists(&self, artifact: &Artifact) -> bool;
}

#[automock]
pub trait TArtifactRepository {
    fn exists(&self, artifact: &Artifact) -> bool;
    fn create(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations ==========================================

impl ArtifactData {
    pub fn new(name: &str, context: &PathBuf, dockerfile: &PathBuf, config: &ArtifactConfig) -> Self {
        Self {
            name: name.to_string(),
            context: context.to_path_buf(),
            dockerfile: dockerfile.to_path_buf(),
            source_url: config.source_repo.clone(),
            source_tag: config.source_tag.clone(),
        }
    }
}

impl Artifact {
    pub fn new(kind: ArtifactKind, source: &PathBuf, dockerfile: &PathBuf, config: &ArtifactConfig) -> Self {
        match kind {
            ArtifactKind::Batcher => Artifact::Batcher(ArtifactData::new("op-batcher", source, dockerfile, config)),
            ArtifactKind::Node => Artifact::Node(ArtifactData::new("op-node", source, dockerfile, config)),
            ArtifactKind::Contracts => Artifact::Contracts(ArtifactData::new(
                "op-contracts",
                source,
                dockerfile,
                config,
            )),
            ArtifactKind::Proposer => Artifact::Proposer(ArtifactData::new("op-proposer", source, dockerfile, config)),
            ArtifactKind::Geth => Artifact::Geth(ArtifactData::new("op-geth", source, dockerfile, config)),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => &data.name,
        }
    }

    pub fn source_info(&self) -> (&str, &str) {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => (&data.source_url, &data.source_tag),
        }
    }

    pub fn context(&self) -> &PathBuf {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => &data.context,
        }
    }

    pub fn dockerfile(&self) -> &PathBuf {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => &data.dockerfile,
        }
    }
}

impl ArtifactFactory {
    pub fn new(project: &Project, config: &CoreConfig) -> Self {
        let mut map: HashMap<ArtifactKind, Arc<Artifact>> = HashMap::new();

        map.insert(
            ArtifactKind::Batcher,
            Arc::new(Artifact::new(
                ArtifactKind::Batcher,
                &project.src.batcher,
                &project.infra.docker.batcher,
                &config.artifacts.batcher,
            )),
        );
        map.insert(
            ArtifactKind::Node,
            Arc::new(Artifact::new(
                ArtifactKind::Node,
                &project.src.node,
                &project.infra.docker.node,
                &config.artifacts.node,
            )),
        );
        map.insert(
            ArtifactKind::Contracts,
            Arc::new(Artifact::new(
                ArtifactKind::Contracts,
                &project.src.contracts,
                &project.infra.docker.contracts,
                &config.artifacts.contracts,
            )),
        );
        map.insert(
            ArtifactKind::Proposer,
            Arc::new(Artifact::new(
                ArtifactKind::Proposer,
                &project.src.proposer,
                &project.infra.docker.proposer,
                &config.artifacts.proposer,
            )),
        );
        map.insert(
            ArtifactKind::Geth,
            Arc::new(Artifact::new(
                ArtifactKind::Geth,
                &project.src.geth,
                &project.infra.docker.geth,
                &config.artifacts.geth,
            )),
        );

        Self { artifacts: map }
    }

    pub fn get(&self, kind: ArtifactKind) -> Arc<Artifact> {
        Arc::clone(self.artifacts.get(&kind).unwrap())
    }

    pub fn get_all(&self) -> Vec<Arc<Artifact>> {
        self.artifacts
            .iter()
            .map(|(_, artifact)| Arc::clone(artifact))
            .collect()
    }
}
