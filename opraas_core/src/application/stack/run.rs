use crate::domain::Deployment;

pub struct InfraRunnerService {
    infra_runner
}

pub trait TInfraRunnerService {
    fn run(contracts_deployment: Deployment);
}

impl TInfraRunnerService for InfraRunnerService {
    fn run(contracts_deployment: Deployment) {
        // ensure infra helm is available

        // copy values.yaml to deployment

        // install helm chart in local kubernettes using set to override 
    }
    
}