use crate::config::{artifacts::ArtifactConfig};
use std::{path::PathBuf};

#[derive(Debug)]
pub struct ArtifactData {
    pub context: PathBuf,
    pub dockerfile: PathBuf,
    pub source_url: String,
    pub source_tag: String,
    pub release_url: String,
    pub release_tag: String,
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

pub trait TArtifactReleaseRepository {
    fn exists(&self, artifact: &Artifact) -> bool;
    fn pull(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
    fn push(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations ==========================================

impl ArtifactData {
    pub fn new(context: &PathBuf, config: &ArtifactConfig) -> Self {
        Self {
            context: context.to_path_buf(),
            dockerfile: "".to_string().into(),
            release_url: config.release_url.clone(),
            release_tag: config.release_tag.clone(),
            source_url: config.image_tag.clone(),
            source_tag: config.image_tag.clone(),
        }
    }
}

impl Artifact {
    pub fn new(kind: ArtifactKind, source: &PathBuf, config: &ArtifactConfig) -> Self {
        match kind {
            ArtifactKind::Batcher => Artifact::Batcher(ArtifactData::new(source, config)),
            ArtifactKind::Node => Artifact::Node(ArtifactData::new(source, config)),
            ArtifactKind::Contracts => Artifact::Contracts(ArtifactData::new(source, config)),
            ArtifactKind::Explorer => Artifact::Explorer(ArtifactData::new(source, config)),
            ArtifactKind::Proposer => Artifact::Proposer(ArtifactData::new(source, config)),
            ArtifactKind::Geth => Artifact::Geth(ArtifactData::new(source, config)),
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

    pub fn release_info(&self) -> (&str, &str) {
        match self {
            Artifact::Batcher(data)
            | Artifact::Node(data)
            | Artifact::Explorer(data)
            | Artifact::Proposer(data)
            | Artifact::Geth(data)
            | Artifact::Contracts(data) => (&data.release_url, &data.release_tag),
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
