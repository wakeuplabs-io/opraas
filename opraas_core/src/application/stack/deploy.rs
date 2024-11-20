use crate::{
    config::CoreConfig,
    domain::{self, Deployment, Project},
    infra::repositories::deployment::InMemoryDeploymentRepository,
};

pub struct StackInfraDeployerService {
    deployment_repository: Box<dyn domain::deployment::TDeploymentRepository>,
}

pub trait TStackInfraDeployerService {
    fn deploy(
        &self,
        name: &str,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>>;
}

// implementations ===================================================

impl StackInfraDeployerService {
    pub fn new(project: &Project) -> Self {
        Self {
            deployment_repository: Box::new(InMemoryDeploymentRepository::new(&project.root)),
        }
    }
}

impl TStackInfraDeployerService for StackInfraDeployerService {
    fn deploy(
        &self,
        deployment_name: &str,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>> {
        let deployment = self.deployment_repository.find(deployment_name)?.unwrap();

        // get zipped artifacts from deployment

        // ensure terraform and helm files are available

        // call terraform to deploy 

        // save outputs if any

        Ok(deployment)
    }
}
