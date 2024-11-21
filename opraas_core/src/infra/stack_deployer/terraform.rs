use crate::domain::{Deployment, Stack};

use super::TStackInfraDeployer;

// deploy deploys infra and saves artifacts
pub struct TerraformDeployer;

impl TerraformDeployer {
    pub fn new() -> Self {
        Self
    }
}

impl TStackInfraDeployer for TerraformDeployer {
    fn deploy(&self, stack: &Stack) -> Result<Deployment, Box<dyn std::error::Error>> {
        todo!()
    }
}