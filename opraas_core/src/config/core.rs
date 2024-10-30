use crate::config::{AccountsConfig, NetworkConfig, SourcesConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub sources: SourcesConfig,
    pub accounts: AccountsConfig,
    pub network: NetworkConfig,
}

impl CoreConfig {
    pub fn new_from_toml<P: AsRef<std::path::Path>>(p: &P) -> Self {
        let config_content = std::fs::read_to_string(p).expect("Failed to read config from toml");
        let config: CoreConfig = toml::from_str(&config_content).expect("Failed to parse config.toml");
    
        config
    }
}
