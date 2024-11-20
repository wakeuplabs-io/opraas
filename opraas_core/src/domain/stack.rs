use std::path::PathBuf;

use super::Project;

pub struct Stack {
    pub helm: PathBuf,
    pub aws: PathBuf,
}

pub trait TStackInfraRepository {
    fn pull(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations ==================================================

impl Stack {
    pub fn new(helm: PathBuf, aws: PathBuf) -> Self {
        Self { helm, aws }
    }

    pub fn from_project(project: &Project) -> Self {
        Self {
            helm: project.infra.helm.clone(),
            aws: project.infra.aws.clone(),
        }
    }
    
}