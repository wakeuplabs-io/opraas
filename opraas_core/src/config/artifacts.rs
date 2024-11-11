use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtifactsConfig {
    pub infra: ArtifactConfig,
    pub node: ArtifactConfig,
    pub geth: ArtifactConfig,
    pub contracts: ArtifactConfig,
    pub batcher: ArtifactConfig,
    pub proposer: ArtifactConfig,
    pub explorer: ArtifactConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArtifactConfig {
    pub release_url: String,
    pub release_tag: String,
    pub image_tag: String,
}

impl ArtifactsConfig {
    pub fn null() -> Self {
        Self {
            infra: ArtifactConfig {
                release_url: String::from("https://github.com/wakeuplabs-io/opraas"),
                release_tag: String::from("v0.0.2"),
                image_tag: String::from("-"),
            },
            node: ArtifactConfig {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-node/v1.9.4"),
                image_tag: String::from("op-node"),
            },
            contracts: ArtifactConfig {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-contracts/v1.6.0"),
                image_tag: String::from("op-contracts"),
            },
            batcher: ArtifactConfig {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-batcher/v1.9.4"),
                image_tag: String::from("op-batcher"),
            },
            proposer: ArtifactConfig {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-proposer/v1.9.4"),
                image_tag: String::from("op-proposer"),
            },
            geth: ArtifactConfig {
                release_url: String::from("https://github.com/ethereum-optimism/op-geth"),
                release_tag: String::from("v1.101315.3"),
                image_tag: String::from("op-geth"),
            },
            explorer: ArtifactConfig {
                release_url: String::from("https://github.com/blockscout/blockscout"),
                release_tag: String::from("v6.9.0-beta"),
                image_tag: String::from("explorer"),
            },
        }
    }
}