use crate::{
    config::get_config_path,
    console::{print_info, style_spinner},
};
use clap::ValueEnum;
use colored::*;
use indicatif::ProgressBar;
use log::info;
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
    contracts_deployer_service: Box<dyn TStackContractsDeployerService>,
    infra_deployer_service: Box<dyn TStackInfraDeployerService>,
}

// implementations ================================================

impl DeployCommand {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();
        Self {
            dialoguer: Box::new(crate::console::Dialoguer::new()),
            contracts_deployer_service: Box::new(StackContractsDeployerService::new(&cwd)),
            infra_deployer_service: Box::new(StackInfraDeployerService::new(&cwd)),
        }
    }

    pub fn run(&self, target: DeployTarget, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        // dev is reserved for local deployments
        if name == "dev" {
            return Err("Name cannot be 'dev'".into());
        } else if name.contains(" ") {
            return Err("Name cannot contain spaces".into());
        }

        // TODO: check if it already exists.

        let registry_url: String = self
            .dialoguer
            .prompt("Input Docker registry url (e.g. dockerhub.io/wakeuplabs) ");
        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let release_factory = ReleaseFactory::new(&project, &config);

        // contracts deployment ===========================================================

        if matches!(target, DeployTarget::Contracts | DeployTarget::All) {
            let contracts_deployer_spinner = style_spinner(ProgressBar::new_spinner(), "Deploying contracts...");

            let contracts_release = release_factory.get(ArtifactKind::Contracts, &release_name, &registry_url);
            self.contracts_deployer_service
                .deploy(&name, &contracts_release, &config)?;

            contracts_deployer_spinner.finish_with_message("Contracts deployed...");
        }

        // infra deployment ===========================================================

        if matches!(target, DeployTarget::Infra | DeployTarget::All) {
            let infra_deployer_spinner = style_spinner(ProgressBar::new_spinner(), "Deploying stack infra...");

            self.infra_deployer_service
                .deploy(&Stack::load(&project, &name))?;

            infra_deployer_spinner.finish_with_message("Infra deployed, your chain is live!");

            print_info("\nFor https domain make sure to create an A record pointing to `elb_dnsname` as specified here: https://github.com/amcginlay/venafi-demos/tree/main/demos/01-eks-ingress-nginx-cert-manager#configure-route53");
        }

        // clear screen and display artifacts ===========================================================

        print!("\x1B[2J\x1B[1;1H");

        if matches!(target, DeployTarget::Contracts | DeployTarget::All) {
            let deployment = self.contracts_deployer_service.find(&name)?;

            if let Some(deployment) = deployment {
                info!("Inspecting contracts deployment: {}", deployment.name);
                deployment.display_contracts_artifacts()?;
            } else {
                return Err("Contracts deployment not found".into());
            }
        }

        if matches!(target, DeployTarget::Infra | DeployTarget::All) {
            let deployment = self.infra_deployer_service.find(&name)?;

            if let Some(deployment) = deployment {
                info!("Inspecting infra deployment: {}", deployment.name);
                deployment.display_infra_artifacts()?;
            } else {
                return Err("Infra deployment not found".into());
            }
        }

        // print instructions

        let bin_name = env!("CARGO_PKG_NAME");

        println!("\n{}\n", "What's Next?".bright_white().bold());

        println!(
            "You can find your deployment artifacts at ./deployments/{}",
            name
        );
        println!("We recommend you keep these files and your keys secure as they're needed to run your deployment.\n");

        println!("Some useful commands for you now:\n");

        println!(
            "  {} {}",
            bin_name.blue(),
            "inspect [contracts|infra|all] --name <deployment_name>".blue()
        );
        println!("    Display the artifacts for each deployment.\n");

        println!(
           "{}", "NOTE: At the moment there's no way to remove a deployment, you'll need to manually go to `infra/aws` and run `terraform destroy`. For upgrades you'll also need to run them directly in helm.".yellow()
        );

        Ok(())
    }
}
