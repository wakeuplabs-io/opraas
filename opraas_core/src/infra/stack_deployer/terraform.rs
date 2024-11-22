use super::TStackInfraDeployer;
use crate::{
    domain::{Deployment, Stack, TDeploymentRepository},
    system, yaml,
};
use std::{collections::HashMap, process::Command};

pub struct TerraformDeployer {
    deployment_repository: Box<dyn TDeploymentRepository>,
}

// implementations ================================================

impl TerraformDeployer {
    pub fn new(root: &std::path::PathBuf) -> Self {
        Self {
            deployment_repository: Box::new(
                crate::infra::repositories::deployment::InMemoryDeploymentRepository::new(root),
            ),
        }
    }
}

impl TStackInfraDeployer for TerraformDeployer {
    fn deploy(&self, stack: &Stack) -> Result<Deployment, Box<dyn std::error::Error>> {
        let mut deployment = stack.deployment.as_ref().unwrap().clone();
        let contracts_artifacts = deployment.contracts_artifacts.as_ref().unwrap();

        // create values file
        let mut updates: HashMap<String, String> = HashMap::new();
        updates.insert(
            "node.config.privateKey".to_string(),
            deployment.accounts_config.sequencer_private_key.clone(),
        );
        updates.insert(
            "batcher.config.privateKey".to_string(),
            deployment.accounts_config.batcher_private_key.clone(),
        );
        updates.insert(
            "proposer.config.privateKey".to_string(),
            deployment.accounts_config.proposer_private_key.clone(),
        );
        updates.insert(
            "node.image.repository".to_string(),
            format!("{}/{}", deployment.registry_url, "op-node"),
        );
        updates.insert(
            "node.image.tag".to_string(),
            deployment.release_name.clone(),
        );
        updates.insert(
            "batcher.image.repository".to_string(),
            format!("{}/{}", deployment.registry_url, "op-batcher"),
        );
        updates.insert(
            "batcher.image.tag".to_string(),
            deployment.release_name.clone(),
        );
        updates.insert(
            "proposer.image.repository".to_string(),
            format!("{}/{}", deployment.registry_url, "op-proposer"),
        );
        updates.insert(
            "proposer.image.tag".to_string(),
            deployment.release_name.clone(),
        );
        updates.insert(
            "geth.image.repository".to_string(),
            format!("{}/{}", deployment.registry_url, "op-geth"),
        );
        updates.insert(
            "geth.image.tag".to_string(),
            deployment.release_name.clone(),
        );
        updates.insert(
            "chain.artifacts".to_string(),
            contracts_artifacts.to_str().unwrap().to_string(),
        );
        updates.insert(
            "chain.l1Rpc".to_string(),
            deployment.network_config.l1_rpc_url.clone(),
        );

        let values = tempfile::NamedTempFile::new()?;
        yaml::rewrite_yaml_to(
            stack.helm.join("values.yaml").to_str().unwrap(),
            values.path().to_str().unwrap(),
            &updates,
        )?;

        system::execute_command(
            Command::new("terraform")
                .arg("init")
                .current_dir(stack.aws.to_str().unwrap()),
            false,
        )?;

        system::execute_command(
            Command::new("terraform")
                .arg("plan")
                .current_dir(stack.aws.to_str().unwrap()),
            false,
        )?;

        system::execute_command(
            Command::new("terraform")
                .arg("apply")
                .arg("-auto-approve")
                .arg(format!(
                    "-var=\"values_file_path={}\"",
                    values.path().to_str().unwrap()
                ))
                .current_dir(stack.aws.to_str().unwrap()),
            false,
        )?;

        let infra_artifacts = tempfile::NamedTempFile::new()?;
        system::execute_command(
            Command::new("terraform")
                .arg("output")
                .arg("-json")
                .arg(">")
                .arg(infra_artifacts.path().to_str().unwrap())
                .current_dir(stack.aws.to_str().unwrap()),
            false,
        )?;

        // set terraform artifacts
        deployment.infra_artifacts = Some(infra_artifacts.path().to_path_buf());
        self.deployment_repository.save(&deployment)?;

        Ok(deployment)
    }
}
