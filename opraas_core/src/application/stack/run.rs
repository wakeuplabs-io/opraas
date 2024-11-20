use std::collections::HashMap;

use crate::domain::{Deployment, Project, TStackInfraRepository};
use crate::infra::repositories::stack::inmemory::InMemoryStackInfraRepository;
use crate::infra::stack_runner::helm::HelmStackRunner;
use crate::infra::stack_runner::stack_runner::TStackRunner;

pub struct StackRunnerService {
    deployment: Deployment,
    project: Project,
    stack_runner: Box<dyn TStackRunner>,
    stack_infra_repository: Box<dyn TStackInfraRepository>,
}

pub trait TStackRunnerService {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations ===================================================

impl StackRunnerService {
    pub fn new(project: &Project, deployment: &Deployment) -> Self {
        Self {
            deployment: deployment.clone(),
            project: project.clone(),
            stack_runner: Box::new(HelmStackRunner::new(
                &project.infra.helm.to_str().unwrap(),
                &format!("op-ruaas-release-{}",&deployment.name),
                &format!("op-ruaas-namespace-{}",&deployment.name),
            )),
            stack_infra_repository: Box::new(InMemoryStackInfraRepository::new(
                &project.infra.root,
            )),
        }
    }
}

impl TStackRunnerService for StackRunnerService {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.stack_infra_repository.exists() == false {
            self.stack_infra_repository.pull()?;
        }

        let mut overrides: HashMap<&str, &str> = HashMap::new();
        overrides.insert(
            "rollup_config",
            self.deployment.rollup_config.to_str().unwrap(),
        );
        overrides.insert(
            "genesis_config",
            self.deployment.genesis_config.to_str().unwrap(),
        );
        overrides.insert(
            "addresses_config",
            self.deployment.addresses_config.to_str().unwrap(),
        );
        overrides.insert(
            "allocs_config",
            self.deployment.allocs_config.to_str().unwrap(),
        );
        overrides.insert(
            "accounts.admin_private_key",
            &self.deployment.accounts_config.admin_private_key,
        );
        overrides.insert(
            "accounts.proposer_private_key",
            &self.deployment.accounts_config.proposer_private_key,
        );
        overrides.insert(
            "accounts.batcher_private_key",
            &self.deployment.accounts_config.batcher_private_key,
        );
        overrides.insert(
            "accounts.sequencer_private_key",
            &self.deployment.accounts_config.sequencer_private_key,
        );
        overrides.insert(
            "accounts.proposer_private_key",
            &self.deployment.accounts_config.proposer_private_key,
        );
        overrides.insert(
            "accounts.sequencer_private_key",
            &self.deployment.accounts_config.sequencer_private_key,
        );
        // other overrides to yaml based on deployment...

        self.stack_runner.run(
            &self
                .project
                .infra
                .helm
                .join("values.yaml")
                .to_str()
                .unwrap(),
            overrides,
        )?;

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stack_runner.stop()?;

        Ok(())
    }
}