use crate::console::{print_info, print_success};
use opraas_core::application::{CreateProjectService, TCreateProjectService};
use std::path::PathBuf;

pub struct NewCommand {
    pub name: String,
    create_project_service: Box<dyn TCreateProjectService>,
}

impl NewCommand {
    pub fn new(name: String) -> Self {
        Self {
            name,
            create_project_service: Box::new(CreateProjectService::new()),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = PathBuf::from(&self.name);

        self.create_project_service.create(&root)?;

        print_success(&format!("✅ Project created at ./{}", self.name));
        print_info("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}
