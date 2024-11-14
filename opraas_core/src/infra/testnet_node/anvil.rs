use super::testnet_node::TTestnetNode;
use crate::system::execute_command;
use std::process::Command;

pub struct AnvilTestnetNode;

const ANVIL_IMAGE: &str = "matzapata/anvil";

// implementations ==============================================

impl AnvilTestnetNode {
    pub fn new() -> Self {
        Self 
    }
}

impl TTestnetNode for AnvilTestnetNode {
    fn start(chain_id: u64, fork_url: &str, port: u64) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(
            Command::new("docker")
                .args([
                    "run",
                    "-d",
                    "-p",
                    &format!("{}:3000", port),
                    "--name",
                    "anvil",
                    ANVIL_IMAGE,
                ])
                .args(["anvil", "--chain-id", &chain_id.to_string(), "--fork", fork_url]),
        )?;

        Ok(())
    }

    fn stop() -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("docker").arg("stop").arg("anvil"))?;

        Ok(())
    }
}
