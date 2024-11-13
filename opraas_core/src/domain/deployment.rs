use std::path::PathBuf;
use crate::config::{AccountsConfig, NetworkConfig};
use super::Release;


pub struct Deployment {
    name: String,
    network_config: NetworkConfig,
    accounts_config: AccountsConfig,
    rollup_config: String,
    artifacts_config: String,
    genesis_config: String,
    releases: Vec<Release>,
}

pub trait TDeploymentRepository {
    fn save(&self, root: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
    fn find(&self, root: &PathBuf, name: String) -> Option<Deployment>;
}

// implementations ======================================================

impl Deployment {
    // pub fn new(name: String) -> Self {
    //     Self { name }
    // }
}