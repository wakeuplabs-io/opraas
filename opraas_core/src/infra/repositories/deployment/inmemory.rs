use std::{fs::File, path::PathBuf};

use crate::{
    config::{AccountsConfig, NetworkConfig},
    domain::{self, Deployment, Release},
    system,
};

pub struct InMemoryDeploymentRepository {
    root: PathBuf,
}

const NETWORK_FILENAME: &str = "network.json";
const ACCOUNTS_FILENAME: &str = "network.json";
const RELEASE_FILENAME: &str = "release.json";
const ROLLUP_FILENAME: &str = "rollup.json";
const GENESIS_FILENAME: &str = "genesis.json";
const ARTIFACTS_FOLDER: &str = "artifacts";

// implementations ====================================

impl InMemoryDeploymentRepository {
    pub fn new(root: &std::path::PathBuf) -> Self {
        Self { root: root.clone() }
    }

    fn load_network_config(
        &self,
        depl_path: &PathBuf,
    ) -> Result<NetworkConfig, Box<dyn std::error::Error>> {
        let reader = File::open(depl_path.join(NETWORK_FILENAME))?;
        let config: NetworkConfig = serde_json::from_reader(reader)?;

        Ok(config)
    }

    fn write_network_config(
        &self,
        depl_path: &PathBuf,
        value: &NetworkConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let writer = File::open(depl_path.join(NETWORK_FILENAME))?;
        serde_json::to_writer(writer, value)?;

        Ok(())
    }

    fn load_accounts_config(
        &self,
        depl_path: &PathBuf,
    ) -> Result<AccountsConfig, Box<dyn std::error::Error>> {
        let reader = File::open(depl_path.join(ACCOUNTS_FILENAME))?;
        let config: AccountsConfig = serde_json::from_reader(reader)?;

        Ok(config)
    }

    fn write_accounts_config(
        &self,
        depl_path: &PathBuf,
        value: &AccountsConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let writer = File::open(depl_path.join(ACCOUNTS_FILENAME))?;
        serde_json::to_writer(writer, value)?;

        Ok(())
    }

    fn load_releases_config(
        &self,
        depl_path: &PathBuf,
    ) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
        let reader = File::open(depl_path.join(RELEASE_FILENAME))?;
        let config: Vec<Release> = serde_json::from_reader(reader)?;

        Ok(config)
    }

    fn write_releases_config(
        &self,
        depl_path: &PathBuf,
        releases: &Vec<Release>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let writer = File::open(depl_path.join(RELEASE_FILENAME))?;
        serde_json::to_writer(writer, releases)?;

        Ok(())
    }

    fn load_path(&self, path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exists = std::fs::exists(&path)?;
        if !exists {
            return Err("Path doesn't exist".into());
        }

        Ok(path.to_path_buf())
    }

    fn write_path(&self, dest: &PathBuf,  src: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        system::copy_and_overwrite(src, dest)?;

        Ok(())
    }
}

impl domain::deployment::TDeploymentRepository for InMemoryDeploymentRepository {
    fn find(&self, name: String) -> Result<Option<domain::Deployment>, Box<dyn std::error::Error>> {
        let depl_path = self.root.join(&name);
        let exists = std::fs::exists(&depl_path).unwrap_or(false);
        if !exists {
            return Ok(None);
        }

        let accounts_config = self.load_accounts_config(&depl_path)?;
        let network_config = self.load_network_config(&depl_path)?;
        let releases = self.load_releases_config(&depl_path)?;

        let artifacts_dir = self.load_path(&depl_path.join(ARTIFACTS_FOLDER))?;
        let rollup_config = self.load_path(&depl_path.join(ROLLUP_FILENAME))?;
        let genesis_config = self.load_path(&depl_path.join(GENESIS_FILENAME))?;

        Ok(Some(Deployment {
            name,
            accounts_config,
            network_config,
            rollup_config,
            genesis_config,
            artifacts_dir,
            releases,
        }))
    }

    fn save(&self, deployment: &Deployment) -> Result<(), Box<dyn std::error::Error>> {
        let depl_path = self.root.join(&deployment.name);
        std::fs::create_dir_all(&depl_path)?;

        self.write_network_config(&depl_path, &deployment.network_config)?;
        self.write_accounts_config(&depl_path, &deployment.accounts_config)?;
        self.write_releases_config(&depl_path, &deployment.releases)?;

        self.write_path(&depl_path.join(ARTIFACTS_FOLDER), &deployment.artifacts_dir)?;
        self.write_path(&depl_path.join(ROLLUP_FILENAME), &deployment.rollup_config)?;
        self.write_path(&depl_path.join(GENESIS_FILENAME), &deployment.genesis_config)?;

        Ok(())
    }
}
