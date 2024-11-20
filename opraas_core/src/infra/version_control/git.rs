use std::process::Command;

use crate::system::execute_command;

use super::version_control::TVersionControl;

pub struct GitVersionControl;

impl GitVersionControl {
    pub fn new() -> Self {
        Self {}
    }
}

impl TVersionControl for GitVersionControl {
    fn init(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("git").arg("init").current_dir(&filepath))?;

        Ok(())
    }

    fn stage(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("git").arg("add").arg(".").current_dir(&filepath))?;

        Ok(())
    }

    fn commit(&self, filepath: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(Command::new("git").arg("commit").arg("-m").arg(message).current_dir(&filepath))?;

        Ok(())
    }
}
