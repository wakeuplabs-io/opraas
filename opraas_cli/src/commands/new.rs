use std::path::PathBuf;
use opraas_core::application::{TCreateProjectService, CreateProjectService};

use crate::console::{print_info, print_success};

pub struct NewCommand {
    pub name: String,
    create_project_service: Box<dyn TCreateProjectService>
}

impl NewCommand {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            create_project_service: Box::new(CreateProjectService::new())
        }
    }

    pub fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let root = PathBuf::from(&self.name);

        self.create_project_service.create(&root)?;

        print_success(&format!("✅ Project created at ./{}", self.name));
        print_info("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}
