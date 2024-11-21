use crate::{
    config::get_config_path,
    console::{print_info, print_warning, style_spinner},
};
use clap::ValueEnum;
use indicatif::ProgressBar;
use opraas_core::{
    application::{
        stack::deploy::{StackInfraDeployerService, TStackInfraDeployerService},
        StackContractsDeployerService, TStackContractsDeployerService,
    },
    config::CoreConfig,
    domain::{ArtifactKind, Project, ReleaseFactory, Stack},
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
            return Err("Name cannot be 'dev'".into());
        }

        let registry_url: String = self
            .dialoguer
            .prompt("Input Docker registry url (e.g. dockerhub.io/wakeuplabs) ");
        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let release_factory = ReleaseFactory::new(&project, &config);

        if matches!(target, DeployTarget::Contracts | DeployTarget::All) {
            if !self.dialoguer.confirm("Do you want to deploy contracts?") {
                print_info("Skipping contracts deployment...");
                return Ok(());
            }

            let contracts_deployer_spinner =
                style_spinner(ProgressBar::new_spinner(), "Deploying contracts...");
            print_warning("This may take a while...");

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
            if !self
                .dialoguer
                .confirm("Are you sure you want to deploy infra?")
            {
                print_info("Skipping infra deployment...");
                return Ok(());
            }

            let infra_deployer_spinner = style_spinner(ProgressBar::new_spinner(), "Deploying stack infra...");

            StackInfraDeployerService::new(&project.root).deploy(&Stack::load(&project, &name))?;

            infra_deployer_spinner.finish_with_message("Infra deployed, your chain is live!");
        }

        Ok(())
    }
}
