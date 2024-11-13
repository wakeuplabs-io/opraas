use std::{fs::File, path::PathBuf};
use tempfile::TempDir;

use crate::{
    config::CoreConfig,
    domain::{self, Deployment, Release},
    infra::{
        self,
        release_runner::DockerArtifactRunner,
        repositories::{
            deployment::InMemoryDeploymentRepository, release::DockerReleaseRepository,
        },
    },
};

pub struct StackContractsDeployerService {
    deployment_repository: Box<dyn domain::deployment::TDeploymentRepository>,
    release_repository: Box<dyn domain::release::TReleaseRepository>,
    release_runner: Box<dyn infra::release_runner::TReleaseRunner>,
}

pub trait TStackContractsDeployerService {
    fn execute(
        &self,
        name: &str,
        contracts_release: &Release,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>>;
}

const IN_NETWORK: &str = "in/network-config.json";
const OUT_ROLLUP: &str = "out/rollup.json";
const OUT_GENESIS: &str = "out/genesis.json";
const OUT_ARTIFACTS: &str = "out/artifacts.json";

// implementations ===================================================

impl StackContractsDeployerService {
    pub fn new(root: &PathBuf) -> Self {
        Self {
            deployment_repository: Box::new(InMemoryDeploymentRepository::new(root)),
            release_repository: Box::new(DockerReleaseRepository::new()),
            release_runner: Box::new(DockerArtifactRunner::new()),
        }
    }
}

impl TStackContractsDeployerService for StackContractsDeployerService {
    fn execute(
        &self,
        name: &str,
        contracts_release: &Release,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        // ensure release is available locally for run
        self.release_repository.pull(&contracts_release)?;

        // we'll create a shared volume to share data with the contracts deployer
        let volume_dir: TempDir = TempDir::new()?; // automatically removed when dropped from scope
        std::fs::create_dir_all(volume_dir.path().join("out"))?;
        std::fs::create_dir_all(volume_dir.path().join("in"))?;

        let rollup_config = volume_dir.path().join(OUT_ROLLUP);
        let genesis_config = volume_dir.path().join(OUT_GENESIS);
        let artifacts_dir = volume_dir.path().join(OUT_ARTIFACTS);
        
        // write network config to shared volume
        let network_config_writer = File::create( volume_dir.path().join(IN_NETWORK))?;
        serde_json::to_writer(network_config_writer, &config.network)?;

        // deployment initially points to local files
        let deployment = Deployment {
            name: name.to_string(),
            network_config: config.network.clone(),
            accounts_config: config.accounts.clone(),
            rollup_config,
            genesis_config,
            artifacts_dir,
            releases: vec![],
        };

        // using contracts artifacts, run to create a deployment
        self.release_runner.run(
            &contracts_release,
            volume_dir.path(),
            vec![
                "-e",
                &format!("IN_NETWORK={}", IN_NETWORK),
                "-e",
                &format!("OUT_ARTIFACTS={}", OUT_ARTIFACTS),
                "-e",
                &format!("OUT_GENESIS={}", OUT_GENESIS),
                "-e",
                &format!("OUT_ROLLUP={}", OUT_ROLLUP),
                "-e",
                &format!("DEPLOYER_ADDRESS={}", &deployment.accounts_config.deployer_address),
                "-e",
                &format!("DEPLOYER_PRIVATE_KEY={}", &deployment.accounts_config.deployer_private_key),
            ],
        )?;

        // save outputs from deployment as well as inputs used for it
        self.deployment_repository.save(&deployment)?;

        Ok(deployment)
    }
}