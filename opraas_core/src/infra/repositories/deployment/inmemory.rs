use crate::{
    config::{AccountsConfig, NetworkConfig},
    domain::{self, Deployment},
    system,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub struct InMemoryDeploymentRepository {
    root: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseMetadata {
    name: String,
    registry_url: String,
}

const NETWORK_FILENAME: &str = "config/network.json";
const ACCOUNTS_FILENAME: &str = "config/accounts.json";
const RELEASE_FILENAME: &str = "config/release.json";
const CONTRACTS_ARTIFACTS_FILENAME: &str = "artifacts/contracts_artifacts.zip";
const INFRA_ARTIFACTS_FILENAME: &str = "artifacts/infra_artifacts.json";

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

    fn load_path(&self, path: &PathBuf) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        let exists = std::fs::exists(&path)?;
        if !exists {
            return Ok(None);
        }

        Ok(Some(path.to_path_buf()))
    }

    fn write_path(&self, dest: &PathBuf, src: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if dest != src {
            system::copy_and_overwrite(src, dest)?;
        }

        Ok(())
    }
}

impl domain::deployment::TDeploymentRepository for InMemoryDeploymentRepository {
    fn find(&self, name: &str) -> Result<Option<domain::Deployment>, Box<dyn std::error::Error>> {
        let depl_path = self.root.join(&name);
        let exists = std::fs::exists(&depl_path).unwrap_or(false);
        if !exists {
            return Ok(None);
        }

        let accounts_config = self.load_accounts_config(&depl_path)?;
        let network_config = self.load_network_config(&depl_path)?;
        let releases = self.load_releases_config(&depl_path)?;

        let infra_artifacts = self.load_path(&depl_path.join(INFRA_ARTIFACTS_FILENAME))?;
        let contracts_artifacts = self.load_path(&depl_path.join(CONTRACTS_ARTIFACTS_FILENAME))?;

        Ok(Some(Deployment {
            name: name.to_string(),
            release_name: releases.name,
            registry_url: releases.registry_url,
            network_config,
            accounts_config,
            infra_artifacts,
            contracts_artifacts,
        }))
    }

    fn save(&self, deployment: &Deployment) -> Result<(), Box<dyn std::error::Error>> {
        let depl_path = self.root.join(&deployment.name);
        std::fs::create_dir_all(&depl_path)?;
        std::fs::create_dir_all(&depl_path.join("artifacts"))?;
        std::fs::create_dir_all(&depl_path.join("config"))?;

        self.write_network_config(&depl_path, &deployment.network_config)?;
        self.write_accounts_config(&depl_path, &deployment.accounts_config)?;
        self.write_releases_config(
            &depl_path,
            &ReleaseMetadata {
                name: deployment.release_name.clone(),
                registry_url: deployment.registry_url.clone(),
            },
        )?;

        if let Some(contracts_artifacts) = &deployment.contracts_artifacts {
            self.write_path(
                &depl_path.join(CONTRACTS_ARTIFACTS_FILENAME),
                contracts_artifacts,
            )?;
        }

        if let Some(infra_artifacts) = &deployment.infra_artifacts {
            self.write_path(&depl_path.join(INFRA_ARTIFACTS_FILENAME), infra_artifacts)?;
        }

        Ok(())
    }
}
