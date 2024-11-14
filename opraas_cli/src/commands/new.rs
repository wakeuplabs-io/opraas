use crate::console::{print_info, print_success};
use opraas_core::application::{CreateProjectService, TCreateProjectService};
use std::path::PathBuf;

pub struct NewCommand {
    create_project_service: Box<dyn TCreateProjectService>,
}

// implementations ================================================

impl NewCommand {
    pub fn new() -> Self {
        Self {
            create_project_service: Box::new(CreateProjectService::new()),
        }
    }

    pub fn run(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let root = PathBuf::from(&name);

        self.create_project_service.create(&root)?;

        print_success(&format!("✅ Project created at ./{}", name));
        print_info("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}
