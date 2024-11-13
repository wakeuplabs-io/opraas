use crate::{
    config::CoreConfig,
    domain::{self, Artifact, Deployment, Project},
    infra::{self, release_runner::DockerArtifactRunner, repositories::{deployment::InMemoryDeploymentRepository, release::DockerReleaseRepository}},
};

pub struct StackContractsDeployerService {
    deployment_repository: Box<dyn domain::deployment::TDeploymentRepository>,
    release_repository: Box<dyn domain::release::TReleaseRepository>,
    release_runner: Box<dyn infra::release_runner::TReleaseRunner>,
}

impl StackContractsDeployerService {
    pub fn new() -> Self {
        Self {
            deployment_repository: Box::new(InMemoryDeploymentRepository::new()),
            release_repository: Box::new(DockerReleaseRepository::new()),
            release_runner: Box::new(DockerArtifactRunner::new()),
        }
    }
}

pub trait TStackContractsDeployerService {
    fn execute(
        &self,
        name: &str,
        project: &Project,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>>;
}

impl TStackContractsDeployerService for StackContractsDeployerService {
    fn execute(
        &self,
        name: &str,
        project: &Project,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        // Deployment contains artifacts images, name
        let contracts_artifact = Artifact::new(
            domain::ArtifactKind::Contracts,
            &project.src.contracts,
            &project.infra.docker.contracts,
            &config.artifacts.contracts,
        );

        Err("TODO:".into())

        // let deployment = Deployment::new(name.to_string());

        // // TODO: get tmp folder
        // // TODO: write config data to it

        // // using contracts artifacts, run to create a deployment
        // self.artifacts_runner.run_artifact(
        //     &contracts_artifact,
        //     "/deployments/.cache",
        //     vec![
        //         "-e",
        //         "ARTIFACTS=out/artifacts.json",
        //         "-e",
        //         "CONFIG=in/deploy-config.json",
        //     ],
        // )?;

        // // write outputs using project repository. Like config and so on
        // self.contracts_deployments_repository
        //     .create_contracts_artifacts(); // "artifacts.json"
        // self.contracts_deployments_repository
        //     .create_network_config();
        // self.contracts_deployments_repository.create_rollup_config();
        // self.contracts_deployments_repository.create_genesis();

        // delete temp folder

        // Ok(deployment)
    }
}
