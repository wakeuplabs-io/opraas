
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountsConfig {
    #[serde(default = "defaults::admin_address")]
    pub admin_address: String,
    #[serde(default = "defaults::admin_private_key")]
    pub admin_private_key: String,
    #[serde(default = "defaults::batcher_address")]
    pub batcher_address: String,
    #[serde(default = "defaults::batcher_private_key")]
    pub batcher_private_key: String,
    #[serde(default = "defaults::sequencer_address")]
    pub sequencer_address: String,
    #[serde(default = "defaults::sequencer_private_key")]
    pub sequencer_private_key: String,
    #[serde(default = "defaults::proposer_address")]
    pub proposer_address: String,
    #[serde(default = "defaults::proposer_private_key")]
    pub proposer_private_key: String,
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
}
