use clap::ValueEnum;
use indicatif::ProgressBar;
use opraas_core::{
    application::{StackContractsDeployerService, TStackContractsDeployerService},
    config::CoreConfig,
    domain::{ArtifactKind, Project, ReleaseFactory},
};

use crate::{
    config::get_config_path,
    console::{print_info, style_spinner},
};

#[derive(Debug, Clone, ValueEnum)]
pub enum DeployTarget {
    Contracts,
    Infra,
    All,
}

pub struct DeployCommand {
    dialoguer: Box<dyn crate::console::TDialoguer>,
}

// implementations ================================================

impl DeployCommand {
    pub fn new() -> Self {
        Self {
            dialoguer: Box::new(crate::console::Dialoguer::new()),
        }
    }

    pub fn run(
        &self,
        target: DeployTarget,
        name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        // dev is reserved for local deployments
        if name == "dev" {
            return Err("Name cannot be 'dev'".into())
        }

        let registry_url: String = self
            .dialoguer
            .prompt("Input Docker registry url (e.g. dockerhub.io/wakeuplabs) ");
        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let release_factory = ReleaseFactory::new(&project, &config);

        if matches!(target, DeployTarget::Contracts | DeployTarget::All) {
            let contracts_deployer_spinner =
                style_spinner(ProgressBar::new_spinner(), "Deploying contracts...");

            let contracts_release =
                release_factory.get(ArtifactKind::Contracts, &release_name, &registry_url);
            StackContractsDeployerService::new(&project).deploy(
                &name,
                &contracts_release,
                &config,
            )?;

            contracts_deployer_spinner.finish_with_message("Contracts deployed to local network");
        }

        if matches!(target, DeployTarget::Infra | DeployTarget::All) {
            print_info("Deploying infra...");
            // find deployment

            // pull terraform files

            // set config to terraform
        }

        Ok(())
    }
}
