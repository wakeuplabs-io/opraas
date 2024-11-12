use async_trait::async_trait;
use log::info;
use std::error::Error;
use std::sync::Arc;
use tokio::signal;
use tokio::task;

use crate::{
    console::print_info,
    testnet::{Anvil, TTestnetNode},
};

pub struct DevCommand {
    testnet_node: Box<dyn TTestnetNode>,
}

impl DevCommand {
    pub fn new() -> Self {
        Self {
            testnet_node: Box::new(Anvil::new()),
        }
    }
}

#[async_trait]
impl crate::Runnable for DevCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let core_cfg = cfg.build_core()?;

        info!("Starting dev environment...");

        // Clone necessary fields from self to avoid borrowing issues
        // let testnet_node = Arc::new(self.testnet_node.as_ref());

        // Spawn a task to run the main work
        let main_task = task::spawn({
            let testnet_node = Arc::new(self.testnet_node.as_ref());
            async move {
                // Start chain fork
                print_info("Starting testnet L1 node...");
                testnet_node.start(1, &core_cfg.core.network.l1_rpc_url);

                // Update config with fork settings
                // Deploy contracts using docker image into fork
                // Deploy infra to local Kubernetes

                Ok::<_, Box<dyn Error + Send + Sync>>(())
            }
        });

        // Spawn a signal handler task for cleanup
        let cleanup_task = task::spawn({
            let testnet_node = Arc::new(self.testnet_node.as_ref());
            async move {
                signal::ctrl_c()
                    .await
                    .expect("Failed to install Ctrl+C handler");
                info!("Interrupt received. Cleaning up...");
                testnet_node.stop();
                info!("Cleanup completed.");
                Ok::<_, Box<dyn Error + Send + Sync>>(())
            }
        });

        // Wait for either the main task to finish or an interrupt signal to be received
        tokio::select! {
            result = main_task => {
                // If the main task completes first, handle any errors it returns
                result?;
            }
            _ = cleanup_task => {
                // If the cleanup task completes (after handling Ctrl+C), exit gracefully
                info!("Exiting after cleanup.");
            }
        }

        Ok(())
    }
}
