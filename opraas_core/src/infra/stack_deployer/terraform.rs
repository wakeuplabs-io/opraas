use super::TStackInfraDeployer;
use crate::{
    domain::{Deployment, Stack, TDeploymentRepository},
    system, yaml,
};
use std::{collections::HashMap, fs, process::Command};

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

    fn create_values_file(&self, stack: &Stack, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut updates: HashMap<&str, String> = HashMap::new();
        let depl = stack.deployment.as_ref().unwrap();

        // global ================================================

        updates.insert("global.storageClassName", "".to_string());

        // private keys ================================================

        updates.insert(
            "node.config.privateKey",
            depl.accounts_config.sequencer_private_key.clone(),
        );
        updates.insert(
            "batcher.config.privateKey",
            depl.accounts_config.batcher_private_key.clone(),
        );
        updates.insert(
            "proposer.config.privateKey",
            depl.accounts_config.proposer_private_key.clone(),
        );

        // artifacts images =============================================

        updates.insert("node.image.tag", depl.release_name.clone());
        updates.insert(
            "node.image.repository",
            format!("{}/{}", depl.registry_url, "op-node"),
        );

        updates.insert("batcher.image.tag", depl.release_name.clone());
        updates.insert(
            "batcher.image.repository",
            format!("{}/{}", depl.registry_url, "op-batcher"),
        );

        updates.insert("proposer.image.tag", depl.release_name.clone());
        updates.insert(
            "proposer.image.repository",
            format!("{}/{}", depl.registry_url, "op-proposer"),
        );

        updates.insert("geth.image.tag", depl.release_name.clone());
        updates.insert(
            "geth.image.repository",
            format!("{}/{}", depl.registry_url, "op-geth"),
        );

        // chain settings ================================================

        updates.insert("chain.id", depl.network_config.l2_chain_id.to_string());
        updates.insert("chain.l1Rpc", depl.network_config.l1_rpc_url.clone());

        // ================================================

        yaml::rewrite_yaml_to(
            stack.helm.join("values.yaml").to_str().unwrap(),
            path,
            &updates,
        )?;

        Ok(())
    }
}

impl TStackInfraDeployer for TerraformDeployer {
    fn deploy(&self, stack: &Stack) -> Result<Deployment, Box<dyn std::error::Error>> {
        let mut deployment = stack.deployment.as_ref().unwrap().clone();

        // create values file
        let values = tempfile::NamedTempFile::new()?;
        self.create_values_file(stack, values.path().to_str().unwrap())?;

        // init, plan and deploy infra

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
                    "-var=values_file_path={}",
                    values.path().to_str().unwrap()
                ))
                .current_dir(stack.aws.to_str().unwrap()),
            false,
        )?;

        // write artifacts to repository

        let infra_artifacts = tempfile::NamedTempFile::new()?;
        let output = system::execute_command(
            Command::new("terraform")
                .arg("output")
                .arg("-json")
                .current_dir(stack.aws.to_str().unwrap()),
            true,
        )?;
        fs::write(infra_artifacts.path(), output)?;

        deployment.infra_artifacts = Some(infra_artifacts.path().to_path_buf());
        self.deployment_repository.save(&deployment)?;

        Ok(deployment)
    }
}
