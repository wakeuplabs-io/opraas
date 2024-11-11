use std::{path::PathBuf, process::Command};
use async_trait::async_trait;

use crate::console::{print_info, print_success};

pub struct NewCommand {
    pub name: String,
    project: Box<dyn crate::utils::system::TSystem>,
}

impl NewCommand {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            system: Box::new(crate::utils::system::System::new()), 
        }
    }
}

#[async_trait]
impl crate::Runnable for NewCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let root = PathBuf::from(&self.name);

        opraas_core::application::project::create_project::ProjectService::new()
            .create(&root)?;

        print_success(&format!("✅ Project created at ./{}", self.name));
        print_info("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}
