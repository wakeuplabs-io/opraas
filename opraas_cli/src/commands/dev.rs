use crate::config::get_config_path;
use crate::console::{print_info, print_success, print_warning};
use ::signal::{trap::Trap, Signal};
use opraas_core::application::stack::run::{StackRunnerService, TStackRunnerService};
use opraas_core::application::{StackContractsDeployerService, TStackContractsDeployerService};
use opraas_core::config::CoreConfig;
use opraas_core::domain::{ArtifactKind, Project, ReleaseFactory, Stack};
use opraas_core::infra::{
    testnet_node::docker::DockerTestnetNode, testnet_node::testnet_node::TTestnetNode,
};

pub struct DevCommand {
    dialoguer: Box<dyn crate::console::TDialoguer>,
    fork_node: Box<dyn TTestnetNode>,
    stack_runner: Box<dyn TStackRunnerService>,
}

// implementations ================================================

impl DevCommand {
    pub fn new() -> Self {
        Self {
            dialoguer: Box::new(crate::console::Dialoguer::new()),
            fork_node: Box::new(DockerTestnetNode::new()),
            stack_runner: Box::new(StackRunnerService::new("opruaas-dev", "opruaas-dev")),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        print_info("Dev command will run a local fork node, deploy contracts to it and then install the infra in your local network.");
        print_info("You can use a release you build with build and release command or a third-party release");

        let registry_url: String = self
            .dialoguer
            .prompt("Input Docker registry url (e.g. dockerhub.io/wakeuplabs) ");
        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let release_factory = ReleaseFactory::new(&project, &config);

        // start local network ===========================

        print_info("⏳ Starting l1 fork...");

        self.fork_node
            .start(config.network.l1_chain_id, &config.network.l1_rpc_url, 8545)?;

        // update config to connect to fork
        let wallet_address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
        let wallet_private_key =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        config.accounts.admin_address = wallet_address.to_string();
        config.accounts.admin_private_key = wallet_private_key.to_string();
        config.accounts.batcher_address = wallet_address.to_string();
        config.accounts.batcher_private_key = wallet_private_key.to_string();
        config.accounts.proposer_address = wallet_address.to_string();
        config.accounts.proposer_private_key = wallet_private_key.to_string();
        config.accounts.sequencer_address = wallet_address.to_string();
        config.accounts.sequencer_private_key = wallet_private_key.to_string();
        config.accounts.deployer_address = wallet_address.to_string();
        config.accounts.deployer_private_key = wallet_private_key.to_string();
        config.accounts.challenger_address = wallet_address.to_string();
        config.accounts.challenger_private_key = wallet_private_key.to_string();
        config.network.l1_rpc_url = "http://host.docker.internal:8545".to_string();

        // Deploy contracts ===========================

        print_info("⏳ Deploying contracts to local network...");

        let contracts_release =
            release_factory.get(ArtifactKind::Contracts, &release_name, &registry_url);
        let contracts_deployer = StackContractsDeployerService::new(&project);
        contracts_deployer.deploy("dev", &contracts_release, &config)?;

        // start stack ===========================

        print_info("⏳ Starting stack...");

        self.stack_runner.start(&Stack::load(&project, "dev"))?;

        // inform results and wait for exit ===========================

        print_success("🚀 All ready...");

        print_info("\n\n================================================\n\n");
        print_info("L1 fork available at http://127.1.1:8545");
        print_info("L2 rpc available at http://127.1.1:8545/rpc");
        print_info("Explorer available at http://127.1.1:8545/rpc");
        print_info("\n\n================================================\n\n");

        print_warning("Press Ctrl + C to exit...");

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

        match self.fork_node.stop() {
            Ok(_) => {}
            Err(e) => {
                print_warning(&format!("Failed to stop fork node: {}", e));
            }
        }

        match self.stack_runner.stop() {
            Ok(_) => {}
            Err(e) => {
                print_warning(&format!("Failed to stop stack runner: {}", e));
            }
        }
    }
}
