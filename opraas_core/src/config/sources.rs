use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SourcesConfig {
    pub node: Source,
    pub geth: Source,
    pub contracts: Source,
    pub batcher: Source,
    pub proposer: Source,
    pub explorer: Source,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    pub base_url: String,
    pub release_tag: String,
}

impl SourcesConfig {
    pub fn null() -> Self {
        Self {
            node: Source {
                base_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-node/v1.9.4"),
            },
            contracts: Source {
                base_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-contracts/v1.9.4"),
            },
            batcher: Source {
                base_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-batcher/v1.9.4"),
            },
            proposer: Source {
                base_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-proposer/v1.9.4"),
            },
            geth: Source {
                base_url: String::from("https://github.com/ethereum-optimism/go-ethereum"),
                release_tag: String::from("v1.10.23"),
            },
            explorer: Source {
                base_url: String::from("https://github.com/ethereum-optimism/op-explorer"),
                release_tag: String::from("v0.1.0"),
            },
        }
    }
}