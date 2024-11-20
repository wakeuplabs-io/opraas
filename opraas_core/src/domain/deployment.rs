use std::path::PathBuf;

use crate::config::{AccountsConfig, NetworkConfig};
use super::Release;

#[derive(Debug, Clone)]
pub struct Deployment {
    pub name: String,
    pub network_config: NetworkConfig,
    pub accounts_config: AccountsConfig,
    pub rollup_config: PathBuf, 
    pub genesis_config: PathBuf,
    pub addresses_config: PathBuf,
    pub allocs_config: PathBuf,
    pub releases: Vec<Release>,
}

pub trait TDeploymentRepository {
    fn save(&self, deployment: &Deployment) -> Result<(), Box<dyn std::error::Error>>;
    fn find(&self, name: String) -> Result<Option<Deployment>, Box<dyn std::error::Error>>;
}
