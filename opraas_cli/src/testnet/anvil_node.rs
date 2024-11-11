use std::process::Command;

use super::TTestnetNode;

pub struct Anvil {
    system: Box<dyn crate::utils::system::TSystem>,
}

impl Anvil {
    pub fn new() -> Self {
        Self {
            system: Box::new(crate::utils::system::System::new()),
        }
    }
}

impl TTestnetNode for Anvil {
    fn start(&self, chain_id: u64, fork_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // self.system.execute_command(
        //     Command::new("docker")
        //         .arg("pull")
        //         .arg("matzapata/anvil")
        // )?;

        self.system.execute_command(
            Command::new("docker")
                .args(["run", "-d", "-p", "8545:3000", "--name", "anvil", "matzapata/anvil"])
                .args(["anvil", "--chain-id", &chain_id.to_string()])
        )?;

        Ok(())
    }
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.execute_command(
            Command::new("docker")
                .arg("stop")
                .arg("anvil")
        )?;

        Ok(())
    }
}