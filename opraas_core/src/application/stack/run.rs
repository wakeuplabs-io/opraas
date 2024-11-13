use crate::domain::Deployment;

pub struct StackRunnerService {
    // stack_runner: 
}

pub trait TStackRunnerService {
    fn run(contracts_deployment: Deployment);
}

impl TStackRunnerService for StackRunnerService {
    fn run(contracts_deployment: Deployment) {
        // ensure infra helm is available

        // copy values.yaml to deployment

        // install helm chart in local kubernettes using set to override 
    }
    
}