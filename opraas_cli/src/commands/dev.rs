use crate::config::get_config_path;
use crate::console::print_info;
use crate::console::print_success;
use crate::console::print_warning;
use opraas_core::config::CoreConfig;
use opraas_core::infra::testnet_node::anvil;
use opraas_core::infra::testnet_node::testnet_node::TTestnetNode;
use ::signal::trap::Trap;
use ::signal::Signal;
use std::process::exit;

pub struct DevCommand {}

impl DevCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();

        print_info("Starting testnet L1 node...");

        let fork_port = 8545;
        let fork_url = format!("http://127.1.1:{}", fork_port);
        anvil::AnvilTestnetNode::start(1, "", 8545).unwrap();
        print_success(&format!("L1 fork available at {}...", fork_url));

        print_info("Deploying contracts to local network...");
        // contracts::StackContractsDeployerService::new(root).execute(name, contracts_release, config);

        // TODO: start helm
        // TODO: hasmap rust with enum

        print_info("Press Ctrl + C to exit...");
        let trap = Trap::trap(&[Signal::SIGINT]);
        for sig in trap {
            match sig {
                Signal::SIGINT => exit(0),
                _ => {}
            }
        }

        print_warning("Ctrl + C received, exiting...");

        Ok(())
    }
}
