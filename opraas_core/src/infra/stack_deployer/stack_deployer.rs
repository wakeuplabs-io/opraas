use crate::domain::{Deployment, Stack};

pub trait TStackInfraDeployer {
    fn deploy(&self, stack: &Stack) -> Result<Deployment, Box<dyn std::error::Error>>;
}
