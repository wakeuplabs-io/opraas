use reqwest::blocking::get;

use super::testnet_node::TTestnetNode;
use crate::system::execute_command;
use std::{process::Command, thread, time};

pub struct DockerTestnetNode {}

const DOCKER_IMAGE: &str = "matzapata/fork-node";
const CONTAINER_NAME: &str = "fork-node";
const MAX_TIMEOUT: u64 = 30;

// implementations ==============================================

impl DockerTestnetNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl TTestnetNode for DockerTestnetNode {
    fn start(&self, chain_id: u32, fork_url: &str, port: u64) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("docker").args(["pull", DOCKER_IMAGE]))?;

        execute_command(Command::new("docker").args([
            "run",
            "-d",
            "-p",
            &format!("{}:8545", port),
            "--name",
            CONTAINER_NAME,
            "-e",
            &format!("FORK_URL={}", fork_url),
            "-e",
            &format!("CHAIN_ID={}", chain_id),
            DOCKER_IMAGE,
        ]))?;

        // Wait for node to start
        let url = format!("http://localhost:{}", port);
        let timeout_duration = time::Duration::from_secs(MAX_TIMEOUT);
        let start_time = time::Instant::now(); 

        loop {
            if start_time.elapsed() >= timeout_duration {
                return Err(format!("Timeout reached: Node did not respond within {} seconds.", MAX_TIMEOUT).into());
            }

            match get(&url) {
                Ok(res) if res.status().is_success() => {
                    // Response is successful, node is ready
                    break;
                }
                Ok(_) => { /*  Response is not successful, retry */ }
                Err(_) => { /* Unable to connect, retry */ }
            }

            // Wait before retrying
            thread::sleep(time::Duration::from_secs(2));
        }

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("docker").arg("stop").arg(CONTAINER_NAME))?;
        execute_command(Command::new("docker").arg("rm").arg(CONTAINER_NAME))?;

        Ok(())
    }
}
