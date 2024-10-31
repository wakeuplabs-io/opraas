use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TreeConfig {
    pub root: PathBuf,
    pub infra: Infra,
    pub src: Src,
}

#[derive(Debug, Clone)]
pub struct  Infra {
    pub root: PathBuf,
    pub aws: PathBuf,
    pub helm: PathBuf,
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

impl TreeConfig {
    pub fn new_from_root(root: PathBuf) -> Self {
        Self {
            root: root.clone(),
            infra: Infra {
                root: root.join("infra"),
                aws: root.join("infra").join("aws"),
                helm: root.join("infra").join("helm"),
            },
            src: Src {
                root: root.join("src"),
                contracts: root.join("src").join("contracts"),
                node: root.join("src").join("node"),
                geth: root.join("src").join("geth"),
                batcher: root.join("src").join("batcher"),
                proposer: root.join("src").join("proposer"),
                explorer: root.join("src").join("explorer"),
            }
        }
    }
}
