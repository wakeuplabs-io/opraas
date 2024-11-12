use crate::config::artifacts::ArtifactConfig;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ArtifactData {
    pub name: String,
    pub context: PathBuf,
    pub dockerfile: PathBuf,
    pub source_url: String,
    pub source_tag: String,
}

pub enum ArtifactKind {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
}

#[derive(Debug)]
pub enum Artifact {
    Batcher(ArtifactData),
    Node(ArtifactData),
    Contracts(ArtifactData),
    Explorer(ArtifactData),
    Proposer(ArtifactData),
    Geth(ArtifactData),
}

pub trait TArtifactSourceRepository {
    fn pull(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
    fn exists(&self, artifact: &Artifact) -> bool;
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
            ArtifactKind::Contracts => Artifact::Contracts(ArtifactData::new("op-contracts",source, dockerfile, config)),
            ArtifactKind::Explorer => Artifact::Explorer(ArtifactData::new("op-explorer",source, dockerfile, config)),
            ArtifactKind::Proposer => Artifact::Proposer(ArtifactData::new("op-proposer",source, dockerfile, config)),
            ArtifactKind::Geth => Artifact::Geth(ArtifactData::new("op-geth",source,dockerfile, config)),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Explorer(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => &data.name,
        }
    }

    pub fn source_info(&self) -> (&str, &str) {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Explorer(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => (&data.source_url, &data.source_tag),
        }
    }

    pub fn context(&self) -> &PathBuf {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Explorer(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => &data.context,
        }
    }

    pub fn dockerfile(&self) -> &PathBuf {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Explorer(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => &data.dockerfile,
        }
    }
}
