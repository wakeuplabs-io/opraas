use crate::{
    domain::{Stack, TStackInfraRepository},
    git,
};

pub struct GitStackInfraRepository {}

// implementations ================================================

impl GitStackInfraRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl TStackInfraRepository for GitStackInfraRepository {
    fn pull(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>> {
        if !stack.helm.exists() {
            git::download_zipped_asset(
                "wakeuplabs-io/op-ruaas",
                "v0.0.3",
                "infra-helm",
                stack.helm.to_str().unwrap(),
            )?;
        }

        if !stack.aws.exists() {
            git::download_zipped_asset(
                "wakeuplabs-io/op-ruaas",
                "v0.0.3",
                "infra-aws",
                stack.aws.to_str().unwrap(),
            )?;
        }

        Ok(())
    }
}
