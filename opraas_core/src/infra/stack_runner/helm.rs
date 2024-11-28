use log::info;

use super::stack_runner::TStackRunner;
use crate::{domain::Stack, system, yaml};
use std::{
    collections::HashMap,
    fs::{self, File},
    process::Command,
};

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

    fn build_dependencies(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>> {
        let repo_dependencies = [
            (
                "ingress-nginx",
                "https://kubernetes.github.io/ingress-nginx",
            ),
            ("cert-manager", "https://charts.jetstack.io/"),
            ("blockscout", "https://blockscout.github.io/helm-charts"),
            (
                "prometheus-community",
                "https://prometheus-community.github.io/helm-charts",
            ),
        ];

        for (repo, url) in repo_dependencies {
            system::execute_command(
                Command::new("helm")
                    .arg("repo")
                    .arg("add")
                    .arg(repo)
                    .arg(url),
                false,
            )?;
        }
        system::execute_command(Command::new("helm").arg("repo").arg("update"), false)?;

        // install pre-requisites, without these helm won't be capable of understanding out chart

        let pre_requisites = [
            ("ingress-nginx", "ingress-nginx/ingress-nginx", vec![]),
            (
                "prometheus",
                "prometheus-community/kube-prometheus-stack",
                vec![],
            ),
            (
                "cert-manager",
                "jetstack/cert-manager",
                vec!["--version", "v1.10.0", "--set", "installCRDs=true"],
            ),
        ];

        for (name, repo, args) in pre_requisites {
            // if already installed skip
            if system::execute_command(Command::new("helm").args(["list", "-n", name]), true)?.contains(name) {
                continue;
            }

            info!("Installing {} from {}", name, repo);
            system::execute_command(
                Command::new("helm")
                    .args(["install", name, repo])
                    .args(args),
                false,
            )?;
        }

        // build dependencies

        system::execute_command(
            Command::new("helm")
                .arg("dependency")
                .arg("build")
                .current_dir(&stack.helm),
            false,
        )?;

        Ok(())
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

    fn wait_for_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Waiting for release to be ready",);

        loop {
            let pods = system::execute_command(
                Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&self.namespace)
                    .arg("--no-headers"),
                true,
            )?;

            if !pods.contains("Pending") && !pods.contains("CrashLoopBackOff") && !pods.contains("Err") {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(2));
        }

        Ok(())
    }
}

impl TStackRunner for HelmStackRunner {
    fn run(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>> {
        let deployment = stack.deployment.as_ref().unwrap();
        let contracts_artifacts = deployment.contracts_artifacts.as_ref().unwrap();

        // add repos, install pre-requisites and build dependencies
        self.build_dependencies(stack)?;

        // create values file from stack
        let values = tempfile::NamedTempFile::new()?;
        self.create_values_file(stack, values.path().to_str().unwrap())?;

        // copy addresses.json and artifacts.zip to helm/config so it can be loaded by it
        let config_dir = stack.helm.join("config");
        fs::create_dir_all(&config_dir)?;

        let unzipped_artifacts = tempfile::TempDir::new()?;
        zip_extract::extract(
            File::open(contracts_artifacts)?,
            &unzipped_artifacts.path(),
            true,
        )?;

        fs::copy(contracts_artifacts, config_dir.join("artifacts.zip"))?;
        fs::copy(
            unzipped_artifacts.path().join("addresses.json"),
            config_dir.join("addresses.json"),
        )?;

        // install core infrastructure

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

        self.wait_for_release()?;

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
