use std::path::PathBuf;
use crate::infra::repositories::deployment::InMemoryDeploymentRepository;

use super::{Deployment, TDeploymentRepository, Project};

pub struct Stack {
    pub helm: PathBuf,
    pub aws: PathBuf,
    pub deployment: Option<Deployment>
}

pub trait TStackInfraRepository {
    fn pull(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations ==================================================

impl Stack {
    pub fn new(helm: PathBuf, aws: PathBuf, deployment: Option<Deployment>) -> Self {
        Self { helm, aws, deployment }
    }

    pub fn load(project: &Project, deployment_name: &str) -> Self {
        let deployment_repository = InMemoryDeploymentRepository::new(&project.root);
        let deployment = deployment_repository.find(deployment_name).unwrap();

        Self {
            helm: project.infra.helm.clone(),
            aws: project.infra.aws.clone(),
            deployment
        }
    }
    
}