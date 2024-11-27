use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountsConfig {
    #[serde(default = "defaults::admin_address")]
    pub admin_address: String,
    #[serde(default = "defaults::admin_private_key", skip_serializing)]
    pub admin_private_key: String,
    #[serde(default = "defaults::batcher_address")]
    pub batcher_address: String,
    #[serde(default = "defaults::batcher_private_key", skip_serializing)]
    pub batcher_private_key: String,
    #[serde(default = "defaults::sequencer_address")]
    pub sequencer_address: String,
    #[serde(default = "defaults::sequencer_private_key", skip_serializing)]
    pub sequencer_private_key: String,
    #[serde(default = "defaults::proposer_address")]
    pub proposer_address: String,
    #[serde(default = "defaults::proposer_private_key", skip_serializing)]
    pub proposer_private_key: String,
    #[serde(default = "defaults::deployer_address")]
    pub deployer_address: String,
    #[serde(default = "defaults::deployer_private_key", skip_serializing)]
    pub deployer_private_key: String,
    #[serde(default = "defaults::challenger_address")]
    pub challenger_address: String,
    #[serde(default = "defaults::challenger_private_key", skip_serializing)]
    pub challenger_private_key: String,
}

mod defaults {
    use std::env;

    // accounts
    pub fn admin_address() -> String {
        env::var("ADMIN_ADDRESS").expect("ADMIN_ADDRESS must be set")
    }
    pub fn admin_private_key() -> String {
        env::var("ADMIN_PRIVATE_KEY").expect("ADMIN_PRIVATE_KEY must be set")
    }
    pub fn batcher_address() -> String {
        env::var("BATCHER_ADDRESS").expect("BATCHER_ADDRESS must be set")
    }
    pub fn batcher_private_key() -> String {
        env::var("BATCHER_PRIVATE_KEY").expect("BATCHER_PRIVATE_KEY must be set")
    }
    pub fn proposer_address() -> String {
        env::var("PROPOSER_ADDRESS").expect("PROPOSER_ADDRESS must be set")
    }
    pub fn proposer_private_key() -> String {
        env::var("PROPOSER_PRIVATE_KEY").expect("PROPOSER_PRIVATE_KEY must be set")
    }
    pub fn sequencer_address() -> String {
        env::var("SEQUENCER_ADDRESS").expect("SEQUENCER_ADDRESS must be set")
    }
    pub fn sequencer_private_key() -> String {
        env::var("SEQUENCER_PRIVATE_KEY").expect("SEQUENCER_PRIVATE_KEY must be set")
    }
    pub fn deployer_address() -> String {
        env::var("DEPLOYER_ADDRESS").expect("DEPLOYER_ADDRESS must be set")
    }
    pub fn deployer_private_key() -> String {
        env::var("DEPLOYER_PRIVATE_KEY").expect("DEPLOYER_PRIVATE_KEY must be set")
    }

    pub fn challenger_address() -> String {
        env::var("CHALLENGER_ADDRESS").expect("CHALLENGER_ADDRESS must be set")
    }
    pub fn challenger_private_key() -> String {
        env::var("CHALLENGER_PRIVATE_KEY").expect("CHALLENGER_PRIVATE_KEY must be set")
    }
}

impl AccountsConfig {
    pub fn null() -> Self {
        Self {
            admin_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            admin_private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            batcher_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            batcher_private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            proposer_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            proposer_private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            sequencer_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            sequencer_private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            deployer_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            deployer_private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            challenger_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            challenger_private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
        }
    }
}
