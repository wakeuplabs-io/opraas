use super::testnet_node::TTestnetNode;
use crate::system::execute_command;
use std::process::Command;

pub struct AnvilTestnetNode {
    chain_id: u32,
    fork_url: String,
    port: u64,
}

const ANVIL_IMAGE: &str = "matzapata/anvil";

// implementations ==============================================

impl TTestnetNode for AnvilTestnetNode {
    fn new(chain_id: u32, fork_url: &str, port: u64) -> Self {
        Self {
            chain_id,
            fork_url: fork_url.to_string(),
            port,
        }
    }

    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(
            Command::new("docker")
                .args([
                    "run",
                    "-d",
                    "-p",
                    &format!("{}:3000", self.port),
                    "--name",
                    "anvil",
                    ANVIL_IMAGE,
                ])
                .args(["anvil", "--chain-id", &self.chain_id.to_string(), "--fork", &self.fork_url]),
        )?;

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("docker").arg("stop").arg("anvil"))?;

        Ok(())
    }
}
