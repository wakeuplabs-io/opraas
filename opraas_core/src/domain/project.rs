use std::{env, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Project {
    pub root: PathBuf,
    pub config: PathBuf,
    pub infra: Infra,
    pub src: Src,
}

#[derive(Debug, Clone)]
pub struct Infra {
    pub root: PathBuf,
    pub aws: PathBuf,
    pub helm: PathBuf,
    pub docker: Dockerfiles,
}

#[derive(Debug, Clone)]
pub struct Dockerfiles {
    pub root: PathBuf,
    pub node: PathBuf,
    pub geth: PathBuf,
    pub batcher: PathBuf,
    pub proposer: PathBuf,
    pub explorer: PathBuf,
    pub contracts: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Src {
    pub root: PathBuf,
    pub contracts: PathBuf,
    pub node: PathBuf,
    pub geth: PathBuf,
    pub batcher: PathBuf,
    pub proposer: PathBuf,
    pub explorer: PathBuf,
}

pub trait TProjectRepository {
    fn write(&self, project: &Project, filepath: &PathBuf, content: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn exists(&self, project: &Project) -> bool;
    fn has(&self, project: &Project, filepath: &PathBuf) -> bool;
}

pub trait TProjectVersionControl {
    fn init(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn stage(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn commit(&self, filepath: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations =================================================================

impl Project {
    /// walk back to find config.toml
    pub fn new_from_cwd() -> Option<Self> {
        let mut current = env::current_dir().unwrap();

        for _ in 0..10 {
            if current.join("config.toml").exists() {
                return Some(Project::new_from_root(current));
            }

            current = current.parent().unwrap().to_path_buf()
        }

        None
    }

    /// creates from given root
    pub fn new_from_root(root: PathBuf) -> Self {
        Self {
            root: root.clone(),
            config: root.join("config.toml"),
            infra: Infra {
                root: root.join("infra"),
                aws: root.join("infra").join("aws"),
                helm: root.join("infra").join("helm"),
                docker: Dockerfiles {
                    root: root.join("infra").join("docker"),
                    node: root.join("infra").join("docker").join("node.dockerfile"),
                    geth: root.join("infra").join("docker").join("geth.dockerfile"),
                    batcher: root.join("infra").join("docker").join("batcher.dockerfile"),
                    proposer: root
                        .join("infra")
                        .join("docker")
                        .join("proposer.dockerfile"),
                    explorer: root
                        .join("infra")
                        .join("docker")
                        .join("explorer.dockerfile"),
                    contracts: root
                        .join("infra")
                        .join("docker")
                        .join("contracts.dockerfile"),
                },
            },
            src: Src {
                root: root.join("src"),
                contracts: root.join("src").join("contracts"),
                node: root.join("src").join("node"),
                geth: root.join("src").join("geth"),
                batcher: root.join("src").join("batcher"),
                proposer: root.join("src").join("proposer"),
                explorer: root.join("src").join("explorer"),
            },
        }
    }
}
