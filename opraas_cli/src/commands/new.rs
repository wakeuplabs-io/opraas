use colored::*;
use indicatif::ProgressBar;
use opraas_core::application::{CreateProjectService, TCreateProjectService};
use std::{env, path::PathBuf};

use crate::{config::BIN_NAME, console::style_spinner};

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
        let mut root = PathBuf::from(&name);
        if !root.is_absolute() {
            root = env::current_dir()?.join(root)
        }

        // create project ============================================

        let create_spinner = style_spinner(
            ProgressBar::new_spinner(),
            &format!("⏳ Creating {} at {}...", name, root.display()),
        );

        self.create_project_service.create(&root)?;

        create_spinner.finish_with_message(format!("✔️ Success! Created {} at {}\n", name, root.display()));

        // print instructions ========================================

        println!("\n{}\n", "What's Next?".bright_white().bold());
        println!("Inside that directory, you can run several commands:\n");

        println!(
            "  {} {}",
            BIN_NAME.blue(),
            "init [contracts|node|etc...]".blue()
        );
        println!("    Initiates artifacts for local builds.\n");

        println!(
            "  {} {}",
            BIN_NAME.blue(),
            "build [contracts|node|etc...]".blue()
        );
        println!("    Builds docker images from artifacts.\n");

        println!(
            "  {} {}",
            BIN_NAME.blue(),
            "release [contracts|node|etc...]".blue()
        );
        println!("    Publishes docker images to be used in dev or prod.\n");

        println!("  {} {}", env!("CARGO_PKG_NAME"), "dev".blue());
        println!("    Runs a local dev environment.\n");

        println!(
            "  {} {}",
            BIN_NAME.blue(),
            "deploy [contracts|infra|all] --name <deployment_name>".blue()
        );
        println!("    Deploys contracts to l1 and infra to kubernetes through terraform.\n");

        println!("We suggest that you begin by typing:\n");
        println!("  {} {}", "cd".blue(), name.blue());
        println!("  {} {}\n", BIN_NAME.blue(), "dev".blue());

        Ok(())
    }
}
