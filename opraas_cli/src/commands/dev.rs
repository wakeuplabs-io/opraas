use crate::config::get_config_path;
use crate::console::{print_info, print_warning, style_spinner};
use ::signal::{trap::Trap, Signal};
use indicatif::ProgressBar;
use opraas_core::application::stack::run::{StackRunnerService, TStackRunnerService};
use opraas_core::application::{StackContractsDeployerService, TStackContractsDeployerService};
use opraas_core::config::CoreConfig;
use opraas_core::domain::{ArtifactKind, Project, ReleaseFactory};
use opraas_core::infra::{
    testnet_node::docker::DockerTestnetNode, testnet_node::testnet_node::TTestnetNode,
};

pub struct DevCommand {
    dialoguer: Box<dyn crate::console::TDialoguer>,
    fork_node: Box<dyn TTestnetNode>,
}

// implementations ================================================

impl DevCommand {
    pub fn new() -> Self {
        Self {
            dialoguer: Box::new(crate::console::Dialoguer::new()),
            fork_node: Box::new(DockerTestnetNode::new()),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        print_info("Dev command will run a local fork node, deploy contracts to it and then install the infra in your local network.");
        print_info("You can use a release you build with build and release command or a third-party release");

        let registry_url: String = self
            .dialoguer
            .prompt("Input Docker registry url (e.g. dockerhub.io/wakeuplabs) ");
        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let release_factory = ReleaseFactory::new(&project, &config);

        // start local network ===========================

        let local_network_spinner =
            style_spinner(ProgressBar::new_spinner(), "Starting local network...");
        self.fork_node
            .start(config.network.l1_chain_id, &config.network.l1_rpc_url, 8545)?;
        local_network_spinner
            .finish_with_message(format!("L1 fork available at http://127.1.1:8545",));

        // Deploy contracts ===========================

        let contracts_deployer_spinner = style_spinner(
            ProgressBar::new_spinner(),
            "Deploying contracts to local network...",
        );
        let contracts_release =
            release_factory.get(ArtifactKind::Contracts, &release_name, &registry_url);
        let contracts_deployer = StackContractsDeployerService::new(&project);
        let deployment = contracts_deployer.deploy("dev", &contracts_release, &config)?;
        contracts_deployer_spinner.finish_with_message("Contracts deployed to local network");

        // start stack ===========================

        // print_info("Starting stack...");
        // let stack_runner = StackRunnerService::new(&project, &deployment);
        // stack_runner.start()?;

        // wait for exit ===========================

        print_info("Press Ctrl + C to exit...");
        let trap = Trap::trap(&[Signal::SIGINT]);
        for sig in trap {
            match sig {
                Signal::SIGINT => {
                    print_warning("Ctrl + C received, exiting...");
                    return Ok(());
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Drop for DevCommand {
    fn drop(&mut self) {
        print_warning("Cleaning up don't interrupt...");

        self.fork_node.stop();
    }
}
