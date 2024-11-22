use log::info;

use super::stack_runner::TStackRunner;
use crate::{domain::Stack, system, yaml};
use std::{collections::HashMap, process::Command};

pub struct HelmStackRunner {
    release_name: String,
    namespace: String,
}

// implementations ============================================================

impl HelmStackRunner {
    pub fn new(release_name: &str, namespace: &str) -> Self {
        Self {
            release_name: release_name.to_string(),
            namespace: namespace.to_string(),
        }
    }
}

impl TStackRunner for HelmStackRunner {
    fn run(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>> {
        let deployment = stack.deployment.as_ref().unwrap();
        let contracts_artifacts = deployment.contracts_artifacts.as_ref().unwrap();

        // install ingress nginx if not available
        let check_output = system::execute_command(
            Command::new("helm").args(["list", "-n", "ingress-nginx"]),
            true,
        )?;
        if !check_output.contains("ingress-nginx") {
            info!("Installing ingress-nginx...");

            system::execute_command(
                Command::new("helm")
                    .arg("repo")
                    .arg("add")
                    .arg("ingress-nginx")
                    .arg("https://kubernetes.github.io/ingress-nginx")
                    .arg("&&")
                    .arg("helm")
                    .arg("repo")
                    .arg("update"),
                false,
            )?;

            system::execute_command(
                Command::new("helm").args([
                    "install",
                    "ingress-nginx",
                    "ingress-nginx/ingress-nginx",
                    "--namespace",
                    "ingress-nginx",
                    "--create-namespace",
                ]),
                true,
            )?;
        }

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
            Command::new("helm")
                .arg("install")
                .arg(format!("op-ruaas-runner-{}", &self.release_name))
                .arg("-f")
                .arg(values.path().to_str().unwrap())
                .arg("--namespace")
                .arg(&self.namespace)
                .arg("--create-namespace")
                .arg(stack.helm.to_str().unwrap()),
            false,
        )?;

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::execute_command(
            Command::new("helm")
                .arg("uninstall")
                .arg(format!("op-ruaas-runner-{}", &self.release_name))
                .arg("--namespace")
                .arg(&self.namespace),
            false,
        )?;

        Ok(())
    }
}
