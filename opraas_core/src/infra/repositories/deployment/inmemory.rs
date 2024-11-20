use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};
use crate::{
    config::{AccountsConfig, NetworkConfig},
    domain::{self, Deployment},
    system,
};

pub struct InMemoryDeploymentRepository {
    root: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseMetadata {
    name: String,
    registry_url: String,
}

const NETWORK_FILENAME: &str = "network.json";
const ACCOUNTS_FILENAME: &str = "accounts.json";
const RELEASE_FILENAME: &str = "release.json";
const ROLLUP_FILENAME: &str = "rollup.json";
const GENESIS_FILENAME: &str = "genesis.json";
const ADDRESSES_FILENAME: &str = "addresses.json";
const ALLOCS_FILENAME: &str = "allocs.json";

// implementations ====================================

impl InMemoryDeploymentRepository {
    pub fn new(root: &std::path::PathBuf) -> Self {
        let deployments_root = root.join("deployments");
        std::fs::create_dir_all(&deployments_root).unwrap();

        Self {
            root: deployments_root,
        }
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
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .open(depl_path.join(NETWORK_FILENAME))?;
        serde_json::to_writer_pretty(writer, value)?;

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
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .open(depl_path.join(ACCOUNTS_FILENAME))?;
        serde_json::to_writer_pretty(writer, value)?;

        Ok(())
    }

    fn load_releases_config(
        &self,
        depl_path: &PathBuf,
    ) -> Result<ReleaseMetadata, Box<dyn std::error::Error>> {
        let reader = File::open(depl_path.join(RELEASE_FILENAME))?;
        let config: ReleaseMetadata = serde_json::from_reader(reader)?;

        Ok(config)
    }

    fn write_releases_config(
        &self,
        depl_path: &PathBuf,
        release_metadata: &ReleaseMetadata,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .open(depl_path.join(RELEASE_FILENAME))?;
        serde_json::to_writer_pretty(writer, release_metadata)?;

        Ok(())
    }

    fn load_path(&self, path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exists = std::fs::exists(&path)?;
        if !exists {
            return Err("Path doesn't exist".into());
        }

        Ok(path.to_path_buf())
    }

    fn write_path(&self, dest: &PathBuf, src: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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

        let addresses_config = self.load_path(&depl_path.join(ADDRESSES_FILENAME))?;
        let rollup_config = self.load_path(&depl_path.join(ROLLUP_FILENAME))?;
        let genesis_config = self.load_path(&depl_path.join(GENESIS_FILENAME))?;
        let allocs_config = self.load_path(&depl_path.join(ALLOCS_FILENAME))?;

        Ok(Some(Deployment {
            name,
            accounts_config,
            network_config,
            rollup_config,
            genesis_config,
            addresses_config,
            allocs_config,
            release_name: releases.name,
            registry_url: releases.registry_url,
        }))
    }

    fn save(&self, deployment: &Deployment) -> Result<(), Box<dyn std::error::Error>> {
        let depl_path = self.root.join(&deployment.name);
        std::fs::create_dir_all(&depl_path)?;

        self.write_network_config(&depl_path, &deployment.network_config)?;
        self.write_accounts_config(&depl_path, &deployment.accounts_config)?;
        self.write_releases_config(
            &depl_path,
            &ReleaseMetadata {
                name: deployment.release_name.clone(),
                registry_url: deployment.registry_url.clone(),
            },
        )?;

        self.write_path(
            &depl_path.join(ADDRESSES_FILENAME),
            &deployment.addresses_config,
        )?;
        self.write_path(&depl_path.join(ROLLUP_FILENAME), &deployment.rollup_config)?;
        self.write_path(
            &depl_path.join(GENESIS_FILENAME),
            &deployment.genesis_config,
        )?;
        self.write_path(&depl_path.join(ALLOCS_FILENAME), &deployment.allocs_config)?;

        Ok(())
    }
}
