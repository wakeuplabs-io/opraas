use crate::config::get_config_path;
use ::signal::{trap::Trap, Signal};
use opraas_core::config::CoreConfig;
use crate::console::{print_info, print_success, print_warning};
use opraas_core::domain::{ArtifactKind, Project, ReleaseFactory};
use opraas_core::application::stack::run::{StackRunnerService, TStackRunnerService};
use opraas_core::application::{StackContractsDeployerService, TStackContractsDeployerService};
use opraas_core::infra::{testnet_node::anvil::AnvilTestnetNode, testnet_node::testnet_node::TTestnetNode};

pub struct DevCommand {
    dialoguer: Box<dyn crate::console::TDialoguer>,
}

impl DevCommand {
    pub fn new() -> Self {
        Self {
            dialoguer: Box::new(crate::console::Dialoguer::new()),
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
        let contracts_release =
            release_factory.get(ArtifactKind::Contracts, &release_name, &registry_url);

        let fork_port = 8545;
        let anvil_node = AnvilTestnetNode::new(
            config.network.l1_chain_id,
            &config.network.l1_rpc_url,
            fork_port,
        );
        anvil_node.start()?;
        print_success(&format!(
            "L1 fork available at http://127.1.1:{}...",
            fork_port
        ));

        print_info("Deploying contracts to local network...");
        let contracts_deployer = StackContractsDeployerService::new(&project);
        let deployment = contracts_deployer.deploy("dev", &contracts_release, &config)?;
        print_success("Contracts deployed to local network");

        print_info("Starting stack...");
        let stack_runner = StackRunnerService::new(&project, &deployment);
        stack_runner.start()?;

        print_info("Press Ctrl + C to exit...");
        let trap = Trap::trap(&[Signal::SIGINT]);
        for sig in trap {
            match sig {
                Signal::SIGINT => {
                    print_warning("Ctrl + C received, exiting...");
                    anvil_node.stop()?;
                    stack_runner.stop()?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
