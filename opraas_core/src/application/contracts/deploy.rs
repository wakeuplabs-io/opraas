use std::fs::File;
use tempfile::TempDir;

use crate::{
    config::CoreConfig,
    domain::{self, Deployment, Project, Release},
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
    fn deploy(
        &self,
        name: &str,
        contracts_release: &Release,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>>;
}

const IN_NETWORK: &str = "in/deploy-config.json";
const OUT_ROLLUP: &str = "out/rollup.json";
const OUT_GENESIS: &str = "out/genesis.json";
const OUT_ADDRESSES: &str = "out/addresses.json";
const OUT_ALLOCS: &str = "out/allocs.json";

// implementations ===================================================

impl StackContractsDeployerService {
    pub fn new(project: &Project) -> Self {
        Self {
            deployment_repository: Box::new(InMemoryDeploymentRepository::new(&project.root)),
            release_repository: Box::new(DockerReleaseRepository::new()),
            release_runner: Box::new(DockerArtifactRunner::new()),
        }
    }
}

impl TStackContractsDeployerService for StackContractsDeployerService {
    fn deploy(
        &self,
        deployment_name: &str,
        contracts_release: &Release,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        // ensure release is available locally for run
        self.release_repository.pull(&contracts_release)?;

        // we'll create a shared volume to share data with the contracts deployer
        let volume_dir: TempDir = TempDir::new()?; // automatically removed when dropped from scope
        std::fs::create_dir_all(volume_dir.path().join("out"))?;
        std::fs::create_dir_all(volume_dir.path().join("in"))?;

        // write network config to shared volume
        let network_config_writer = File::create( volume_dir.path().join(IN_NETWORK))?;
        serde_json::to_writer(network_config_writer, &config.network)?;

        // deployment initially points to local files
        let deployment = Deployment {
            name: deployment_name.to_string(),
            network_config: config.network.clone(),
            accounts_config: config.accounts.clone(),
            rollup_config: volume_dir.path().join(OUT_ROLLUP),
            genesis_config: volume_dir.path().join(OUT_GENESIS),
            addresses_config: volume_dir.path().join(OUT_ADDRESSES),
            allocs_config: volume_dir.path().join(OUT_ALLOCS),
            releases: vec![contracts_release.clone()],
        };

        // using contracts artifacts, run to create a deployment
        self.release_runner.run(
            &contracts_release,
            volume_dir.path(),
            vec![
                "-e",
                &format!("ETH_RPC_URL={}", deployment.network_config.l1_rpc_url),
                "-e",
                &format!("IMPL_SALT={}", OUT_ADDRESSES),
                "-e",
                &format!("DEPLOYER_ADDRESS={}", deployment.accounts_config.deployer_address),
                "-e",
                &format!("DEPLOYER_PRIVATE_KEY={}", deployment.accounts_config.deployer_private_key),
            ],
        )?;

        // save outputs from deployment as well as inputs used for it
        self.deployment_repository.save(&deployment)?;

        Ok(deployment)
    }
}
