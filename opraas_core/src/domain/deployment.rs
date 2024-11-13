use std::path::PathBuf;



pub struct Deployment {
    name: String
}

pub trait TDeploymentRepository {
    fn create_contracts_artifacts(&self);
    fn get_contracts_artifacts(&self, name: &str) -> PathBuf;
    fn create_helm_values(&self) -> PathBuf;
    fn get_helm_values(&self) -> PathBuf;
}

// implementations ======================================================

impl Deployment {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}