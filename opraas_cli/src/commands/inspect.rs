use clap::ValueEnum;
use log::info;
use opraas_core::application::{
    stack::deploy::{StackInfraDeployerService, TStackInfraDeployerService},
    StackContractsDeployerService, TStackContractsDeployerService,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum InspectTarget {
    Contracts,
    Infra,
    All,
}

pub struct InspectCommand {
    contracts_deployer_service: Box<dyn TStackContractsDeployerService>,
    infra_deployer_service: Box<dyn TStackInfraDeployerService>,
}

// implementations ===================================================

impl InspectCommand {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();
        Self {
            contracts_deployer_service: Box::new(StackContractsDeployerService::new(&cwd)),
            infra_deployer_service: Box::new(StackInfraDeployerService::new(&cwd)),
        }
    }

    pub fn run(&self, target: InspectTarget, deployment_name: String) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Inspecting deployment: {}, target: {:?}",
            deployment_name, target
        );

        if matches!(target, InspectTarget::Contracts | InspectTarget::All) {
            let deployment = self.contracts_deployer_service.find(&deployment_name)?;

            if let Some(deployment) = deployment {
                println!("{}", deployment.display_contracts_artifacts()?);
            } else {
                return Err("Contracts deployment not found".into());
            }
        }

        if matches!(target, InspectTarget::Infra | InspectTarget::All) {
            let deployment = self.infra_deployer_service.find(&deployment_name)?;

            if let Some(deployment) = deployment {
                println!("{}", deployment.display_infra_artifacts()?);
            } else {
                return Err("Infra deployment not found".into());
            }
        }

        Ok(())
    }
}
