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
use rand::Rng;
use std::collections::HashMap;
use tempfile::TempDir;

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

    fn find(&self, name: &str) -> Result<Option<Deployment>, Box<dyn std::error::Error>>;
}

const IN_NETWORK: &str = "in/deploy-config.json";
const OUT_ARTIFACTS: &str = "out/artifacts.zip";

// implementations ===================================================

impl StackContractsDeployerService {
    pub fn new(root: &std::path::PathBuf) -> Self {
        Self {
            deployment_repository: Box::new(InMemoryDeploymentRepository::new(root)),
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
        let volume = volume_dir.path();

        // deployment initially points to local files
        let mut deployment = Deployment::new(
            deployment_name.to_string(),
            contracts_release.artifact_tag.clone(),
            contracts_release.registry_url.clone(),
            config.network.clone(),
            config.accounts.clone(),
        );

        // write contracts config to shared volume for artifact consumption
        deployment.write_contracts_config(&volume_dir.path().join(IN_NETWORK))?;

        // create environment
        let mut env: HashMap<&str, String> = HashMap::new();
        env.insert("ETH_RPC_URL", config.network.l1_rpc_url.clone());
        env.insert("DEPLOYER_ADDRESS", config.accounts.deployer_address.clone());
        env.insert(
            "DEPLOYER_PRIVATE_KEY",
            config.accounts.deployer_private_key.clone(),
        );
        env.insert(
            "IMPL_SALT",
            rand::thread_rng()
                .gen::<[u8; 16]>()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>(),
        );

        // using contracts artifact, create a deployment
        self.release_runner.run(&contracts_release, volume, env)?;

        // check out zip exists and add it to deployment
        let artifact_path = volume_dir.path().join(OUT_ARTIFACTS);
        if !artifact_path.exists() {
            return Err("Contracts artifact not found".into());
        }
        deployment.contracts_artifacts = Some(artifact_path);
        self.deployment_repository.save(&deployment)?;

        Ok(deployment)
    }

    fn find(&self, name: &str) -> Result<Option<Deployment>, Box<dyn std::error::Error>> {
        self.deployment_repository.find(name)
    }
}
