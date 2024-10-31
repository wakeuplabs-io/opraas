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
    pub release_url: String,
    pub release_tag: String,
}

impl SourcesConfig {
    pub fn null() -> Self {
        Self {
            node: Source {
                release_url: String::from("https://github.com/ethereum-optimism/op-node"),
                release_tag: String::from("v0.1.0"),
            },
            geth: Source {
                release_url: String::from("https://github.com/ethereum-optimism/go-ethereum"),
                release_tag: String::from("v1.10.23"),
            },
            contracts: Source {
                release_url: String::from("https://github.com/ethereum-optimism/op-contracts"),
                release_tag: String::from("v0.1.0"),
            },
            batcher: Source {
                release_url: String::from("https://github.com/ethereum-optimism/optimism"),
                release_tag: String::from("op-batcher/v1.9.4"),
            },
            proposer: Source {
                release_url: String::from("https://github.com/ethereum-optimism/op-proposer"),
                release_tag: String::from("v0.1.0"),
            },
            explorer: Source {
                release_url: String::from("https://github.com/ethereum-optimism/op-explorer"),
                release_tag: String::from("v0.1.0"),
            },
        }
    }
}