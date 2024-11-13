use crate::console::print_info;
use crate::console::print_success;
use std::time::Duration;
use opraas_core::application::contracts;
use opraas_core::infra::testnet_node::anvil;
use opraas_core::infra::testnet_node::testnet_node::TTestnetNode;
use tokio::signal;
use tokio::task;

pub struct DevCommand {}

impl DevCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // enter docker repo
        // enter release name you want to run

        // deployment from release

        // Spawn a task to run the main work
        let main_task = task::spawn({
            async move {
                print_info("Starting testnet L1 node...");
                std::thread::sleep(Duration::new(10, 0)); // 10 seconds and 0 nanoseconds

                // start local network
                let fork_port = 8545;
                let fork_url = format!("http://127.1.1:{}", fork_port);
                anvil::AnvilTestnetNode::start(1, "", 8545).unwrap();
                print_success(&format!("L1 fork available at {}...", fork_url));

                // TODO: deploy contracts to local network
                print_info("Deploying contracts to local network...");
                // contracts::StackContractsDeployerService::new()

                // Deploy infra to local Kubernetes

                // Ok(())
            }
        });

        // Spawn a signal handler task for cleanup
        let cleanup_task = task::spawn({
            async move {
                signal::ctrl_c()
                    .await
                    .expect("Failed to install Ctrl+C handler");                
            }
        });

        // Wait for either the main task to finish or an interrupt signal to be received
        tokio::select! {
            _ = main_task => {},
            _ = cleanup_task => {
                print_info("Exiting...");

                // cleanup tasks
                anvil::AnvilTestnetNode::stop().unwrap();

                print_success("Successfully exited");
                print_info("If you're ready for deployment run `release` and `deploy` commands");
            }
        }

        Ok(())
    }
}
