use crate::{
    domain::{self, Deployment, Stack},
    infra::{self, repositories::stack_infra::GitStackInfraRepository, stack_deployer::TerraformDeployer},
};

pub struct StackInfraDeployerService {
    stack_deployer: Box<dyn infra::stack_deployer::TStackInfraDeployer>,
    stack_infra_repository: Box<dyn domain::stack::TStackInfraRepository>,
}

pub trait TStackInfraDeployerService {
    fn deploy(
        &self,
        stack: &Stack,
    ) -> Result<Deployment, Box<dyn std::error::Error>>;
}

// implementations ===================================================

impl StackInfraDeployerService {
    pub fn new(root: &std::path::PathBuf) -> Self {
        Self {
            stack_deployer: Box::new(TerraformDeployer::new(root)),
            stack_infra_repository: Box::new(GitStackInfraRepository::new()),
        }
    }
}

impl TStackInfraDeployerService for StackInfraDeployerService {
    fn deploy(
        &self,
        stack: &Stack,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        if stack.deployment.is_none() {
            return Err("Stack does not contain deployment".into());
        }

        self.stack_infra_repository.pull(stack)?;
        
        let deployment = self.stack_deployer.deploy(stack)?;

        Ok(deployment)
    }
}
