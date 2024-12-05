use crate::{
    config::{
        SystemRequirementsChecker, TSystemRequirementsChecker, DOCKER_REQUIREMENT, HELM_REQUIREMENT, K8S_REQUIREMENT,
        TERRAFORM_REQUIREMENT,
    },
    infra::console::{print_info, style_spinner, Dialoguer, TDialoguer},
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
    domain::{ArtifactFactory, ArtifactKind, ProjectFactory, Release, Stack, TArtifactFactory, TProjectFactory},
    infra::{
        deployment::InMemoryDeploymentRepository,
        release::{DockerReleaseRepository, DockerReleaseRunner},
        stack::{deployer_terraform::TerraformDeployer, repo_inmemory::GitStackInfraRepository},
    },
};

#[derive(Debug, Clone, ValueEnum)]
pub enum DeployTarget {
    Contracts,
    Infra,
    All,
}

pub struct DeployCommand {
    dialoguer: Box<dyn TDialoguer>,
    contracts_deployer: Box<dyn TStackContractsDeployerService>,
    infra_deployer: Box<dyn TStackInfraDeployerService>,
    system_requirement_checker: Box<dyn TSystemRequirementsChecker>,
    artifacts_factory: Box<dyn TArtifactFactory>,
    project_factory: Box<dyn TProjectFactory>,
}

// implementations ================================================

impl DeployCommand {
    pub fn new() -> Self {
        let project_factory = Box::new(ProjectFactory::new());
        let project = project_factory.from_cwd().unwrap();

        Self {
            dialoguer: Box::new(Dialoguer::new()),
            contracts_deployer: Box::new(StackContractsDeployerService::new(
                Box::new(InMemoryDeploymentRepository::new(&project.root)),
                Box::new(DockerReleaseRepository::new()),
                Box::new(DockerReleaseRunner::new()),
            )),
            infra_deployer: Box::new(StackInfraDeployerService::new(
                Box::new(TerraformDeployer::new(&project.root)),
                Box::new(GitStackInfraRepository::new()),
                Box::new(InMemoryDeploymentRepository::new(&project.root)),
            )),
            system_requirement_checker: Box::new(SystemRequirementsChecker::new()),
            artifacts_factory: Box::new(ArtifactFactory::new()),
            project_factory,
        }
    }

    pub fn run(
        &self,
        target: DeployTarget,
        name: String,
        deploy_deterministic_deployer: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.system_requirement_checker.check(vec![
            DOCKER_REQUIREMENT,
            K8S_REQUIREMENT,
            HELM_REQUIREMENT,
            TERRAFORM_REQUIREMENT,
        ])?;

        let project = self.project_factory.from_cwd().unwrap();
        let config = CoreConfig::new_from_toml(&project.config).unwrap();

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

        // contracts deployment ===========================================================

        if matches!(target, DeployTarget::Contracts | DeployTarget::All) {
            let contracts_deployer_spinner = style_spinner(ProgressBar::new_spinner(), "Deploying contracts...");

            let contracts_release = Release::from_artifact(
                &self
                    .artifacts_factory
                    .get(&ArtifactKind::Contracts, &project, &config),
                &release_name,
                &registry_url,
            );
            self.contracts_deployer.deploy(
                &name,
                &contracts_release,
                &config,
                deploy_deterministic_deployer,
                true,
            )?;

            contracts_deployer_spinner.finish_with_message("✔️ Contracts deployed...");
        }

        // infra deployment ===========================================================

        if matches!(target, DeployTarget::Infra | DeployTarget::All) {
            let infra_deployer_spinner = style_spinner(ProgressBar::new_spinner(), "Deploying stack infra...");

            self.infra_deployer.deploy(&Stack::load(&project, &name))?;

            infra_deployer_spinner.finish_with_message("✔️ Infra deployed, your chain is live!");

            print_info("\nFor https domain make sure to create an A record pointing to `elb_dnsname` as specified here: https://github.com/amcginlay/venafi-demos/tree/main/demos/01-eks-ingress-nginx-cert-manager#configure-route53");
        }

        // clear screen and display artifacts ===========================================================

        print!("\x1B[2J\x1B[1;1H");

        if matches!(target, DeployTarget::Contracts | DeployTarget::All) {
            let deployment = self.contracts_deployer.find(&name)?;

            if let Some(deployment) = deployment {
                info!("Inspecting contracts deployment: {}", deployment.name);
                deployment.display_contracts_artifacts()?;
            } else {
                return Err("Contracts deployment not found".into());
            }
        }

        if matches!(target, DeployTarget::Infra | DeployTarget::All) {
            let deployment = self.infra_deployer.find(&name)?;

            if let Some(deployment) = deployment {
                info!("Inspecting infra deployment: {}", deployment.name);
                deployment.display_infra_artifacts()?;
            } else {
                return Err("Infra deployment not found".into());
            }
        }

        // print instructions

        println!(
            "\n{title}\n\n\
            You can find your deployment artifacts at ./deployments/{name}\n\n\
            We recommend you keep these files and your keys secure as they're needed to run your deployment.\n\n\
            Some useful commands for you now:\n\n\
            - {bin_name} {command}\n\
            \tDisplay the artifacts for each deployment.\n\n\
            {note}\n",
            title = "What's Next?".bright_white().bold(),
            bin_name=env!("CARGO_BIN_NAME").blue(),
            command="inspect [contracts|infra|all] --name <deployment_name>".blue(),
            note="NOTE: At the moment there's no way to remove a deployment, you'll need to manually go to `infra/aws` and run `terraform destroy`. For upgrades you'll also need to run them directly in helm.".yellow()
        );

        Ok(())
    }
}
