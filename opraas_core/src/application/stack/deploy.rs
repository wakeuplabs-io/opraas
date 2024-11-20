use std::path::PathBuf;

use crate::{
    config::CoreConfig,
    domain::{self, Deployment, Project, Stack},
    infra::repositories::deployment::InMemoryDeploymentRepository,
};

pub struct StackInfraDeployerService {
    deployment_repository: Box<dyn domain::deployment::TDeploymentRepository>,
}

pub trait TStackInfraDeployerService {
    fn deploy(
        &self,
        stack: &Stack,
        name: &str,
        config: &CoreConfig,
    ) -> Result<Deployment, Box<dyn std::error::Error>>;
}

// implementations ===================================================

impl StackInfraDeployerService {
    pub fn new(root: &PathBuf) -> Self {
        Self {
            deployment_repository: Box::new(InMemoryDeploymentRepository::new(root)),
        }
    }
}

impl TStackInfraDeployerService for StackInfraDeployerService {
    fn deploy(
        &self,
        stack: &Stack,
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
