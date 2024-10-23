use opraas_core::config::{AccountsConfig, NetworkConfig, SourcesConfig};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: SourcesConfig,
    pub accounts: AccountsConfig,
    pub network: NetworkConfig,
}

pub fn load_config() -> Config {
    let config_content = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_content).expect("Failed to parse config.toml");

    config
}
