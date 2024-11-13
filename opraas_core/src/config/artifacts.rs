use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtifactsConfig {
    pub infra: Artifact,
    pub node: Artifact,
    pub geth: Artifact,
    pub contracts: Artifact,
    pub batcher: Artifact,
    pub proposer: Artifact,
    pub explorer: Artifact,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Artifact {
    pub release_url: String,
    pub release_tag: String,
    pub image_tag: String,
}

impl ArtifactsConfig {
    pub fn null() -> Self {
        Self {
            infra: Artifact {
                release_url: String::from("https://github.com/wakeuplabs-io/opraas"),
                release_tag: String::from("v0.0.2"),
                image_tag: String::from("-"),
            },
            node: Artifact {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-node/v1.9.4"),
                image_tag: String::from("op-node"),
            },
            contracts: Artifact {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-contracts/v1.6.0"),
                image_tag: String::from("op-contracts"),
            },
            batcher: Artifact {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-batcher/v1.9.4"),
                image_tag: String::from("op-batcher"),
            },
            proposer: Artifact {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-proposer/v1.9.4"),
                image_tag: String::from("op-proposer"),
            },
            geth: Artifact {
                release_url: String::from("https://github.com/ethereum-optimism/op-geth"),
                release_tag: String::from("v1.101315.3"),
                image_tag: String::from("op-geth"),
            },
            explorer: Artifact {
                release_url: String::from("https://github.com/blockscout/blockscout"),
                release_tag: String::from("v6.9.0-beta"),
                image_tag: String::from("explorer"),
            },
        }
    }
}