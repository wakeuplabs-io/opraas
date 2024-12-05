use clap::ValueEnum;
use opraas_core::{
    application::{
        stack::deploy::{StackInfraDeployerService, TStackInfraDeployerService},
        StackContractsDeployerService, TStackContractsDeployerService,
    },
    domain::Project,
    infra::{
        release_runner::DockerReleaseRunner,
        repositories::{
            deployment::InMemoryDeploymentRepository, release::DockerReleaseRepository,
            stack_infra::GitStackInfraRepository,
        },
        stack_deployer::TerraformDeployer,
    },
};

#[derive(Debug, Clone, ValueEnum)]
pub enum InspectTarget {
    Contracts,
    Infra,
    All,
}

pub struct InspectCommand {
    contracts_deployer: Box<dyn TStackContractsDeployerService>,
    infra_deployer: Box<dyn TStackInfraDeployerService>,
}

// implementations ===================================================

impl InspectCommand {
    pub fn new() -> Self {
        let project = Project::new_from_cwd().unwrap();

        Self {
            contracts_deployer: Box::new(StackContractsDeployerService::new(
                Box::new(InMemoryDeploymentRepository::new(&project.root)),
                Box::new(DockerReleaseRepository::new()),
                Box::new(DockerReleaseRunner::new()),
            )),
            infra_deployer: Box::new(StackInfraDeployerService::new(
                Box::new(TerraformDeployer::new(&project.root)),
                Box::new(GitStackInfraRepository::new()),
                Box::new(InMemoryDeploymentRepository::new(&project.root)),
            )),
        }
    }

    pub fn run(&self, target: InspectTarget, deployment_name: String) -> Result<(), Box<dyn std::error::Error>> {
        if matches!(target, InspectTarget::Contracts | InspectTarget::All) {
            let deployment = self.contracts_deployer.find(&deployment_name)?;

            if let Some(deployment) = deployment {
                println!("{}", deployment.display_contracts_artifacts()?);
            } else {
                return Err("Contracts deployment not found".into());
            }
        }

        if matches!(target, InspectTarget::Infra | InspectTarget::All) {
            let deployment = self.infra_deployer.find(&deployment_name)?;

            if let Some(deployment) = deployment {
                println!("{}", deployment.display_infra_artifacts()?);
            } else {
                return Err("Infra deployment not found".into());
            }
        }

        Ok(())
    }
}
