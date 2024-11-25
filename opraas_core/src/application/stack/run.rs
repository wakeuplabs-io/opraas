use crate::domain::{Stack, TStackInfraRepository};
use crate::infra::repositories::stack_infra::inmemory::GitStackInfraRepository;
use crate::infra::stack_runner::helm::HelmStackRunner;
use crate::infra::stack_runner::stack_runner::TStackRunner;

pub struct StackRunnerService {
    stack_runner: Box<dyn TStackRunner>,
    stack_infra_repository: Box<dyn TStackInfraRepository>,
}

pub trait TStackRunnerService {
    fn start(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}

// implementations ===================================================

impl StackRunnerService {
    pub fn new(release_name: &str, namespace: &str) -> Self {
        Self {
            stack_runner: Box::new(HelmStackRunner::new(release_name, namespace)),
            stack_infra_repository: Box::new(GitStackInfraRepository::new()),
        }
    }
}

impl TStackRunnerService for StackRunnerService {
    fn start(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>> {
        self.stack_infra_repository.pull(stack)?;

        self.stack_runner.run(stack)?;

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stack_runner.stop()?;

        Ok(())
    }
}
