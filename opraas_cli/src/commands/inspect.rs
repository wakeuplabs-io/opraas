use clap::ValueEnum;
use opraas_core::{
    application::{
        contracts::{
            StackContractsDeployerService, StackContractsInspectorService, TStackContractsDeployerService,
            TStackContractsInspectorService,
        },
        stack::{
            StackInfraDeployerService, StackInfraInspectorService, TStackInfraDeployerService,
            TStackInfraInspectorService,
        },
    },
    domain::{ProjectFactory, TProjectFactory},
    infra::{
        deployment::InMemoryDeploymentRepository,
        release::{DockerReleaseRepository, DockerReleaseRunner},
        stack::{deployer_terraform::TerraformDeployer, repo_inmemory::GitStackInfraRepository},
    },
};
use std::io::Cursor;

#[derive(Debug, Clone, ValueEnum)]
pub enum InspectTarget {
    Contracts,
    Infra,
    All,
}

pub struct InspectCommand {
    contracts_deployer: Box<dyn TStackContractsDeployerService>,
    contracts_inspector: Box<dyn TStackContractsInspectorService>,
    infra_deployer: Box<dyn TStackInfraDeployerService>,
    infra_inspector: Box<dyn TStackInfraInspectorService>,
}

// implementations ===================================================

impl InspectCommand {
    pub fn new() -> Self {
        let project_factory = Box::new(ProjectFactory::new());
        let project = project_factory.from_cwd().unwrap();

        Self {
            contracts_deployer: Box::new(StackContractsDeployerService::new(
                Box::new(InMemoryDeploymentRepository::new(&project.root)),
                Box::new(DockerReleaseRepository::new()),
                Box::new(DockerReleaseRunner::new()),
            )),
            contracts_inspector: Box::new(StackContractsInspectorService::new()),
            infra_deployer: Box::new(StackInfraDeployerService::new(
                Box::new(TerraformDeployer::new(&project.root)),
                Box::new(GitStackInfraRepository::new()),
                Box::new(InMemoryDeploymentRepository::new(&project.root)),
            )),
            infra_inspector: Box::new(StackInfraInspectorService::new()),
        }
    }

    pub fn run(&self, target: InspectTarget, deployment_name: String) -> Result<(), Box<dyn std::error::Error>> {
        if matches!(target, InspectTarget::Contracts | InspectTarget::All) {
            let deployment = self.contracts_deployer.find(&deployment_name)?;

            if let Some(deployment) = deployment {
                let artifact_cursor = Cursor::new(std::fs::read(&deployment.contracts_artifacts.unwrap())?);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&self.contracts_inspector.inspect(artifact_cursor)?)?
                );
            } else {
                return Err("Contracts deployment not found".into());
            }
        }

        if matches!(target, InspectTarget::Infra | InspectTarget::All) {
            let deployment = self.infra_deployer.find(&deployment_name)?;

            if let Some(deployment) = deployment {
                let artifact_cursor = Cursor::new(std::fs::read(&deployment.infra_artifacts.unwrap())?);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&self.infra_inspector.inspect(artifact_cursor)?)?
                );
            } else {
                return Err("Infra deployment not found".into());
            }
        }

        Ok(())
    }
}
