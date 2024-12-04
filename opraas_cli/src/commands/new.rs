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

        create_spinner.finish_with_message(format!(
            "✔️ Success! Created {} at {}\n",
            name,
            root.display()
        ));

        // print instructions ========================================

        println!(
            "\n{title}\n\n\
            Inside that directory, you can run several commands:\n\n\
            - {bin} {init_cmd}\n\
            \tInitiates artifacts for local builds.\n\n\
            - {bin} {build_cmd}\n\
            \tBuilds docker images from artifacts.\n\n\
            - {bin} {release_cmd}\n\
            \tPublishes docker images to be used in dev or prod.\n\n\
            - {bin} {dev_cmd}\n\
            \tRuns a local dev environment.\n\n\
            - {bin} {deploy_cmd}\n\
            \tDeploys contracts to l1 and infra to kubernetes through terraform.\n\n\
            We suggest that you begin by typing:\n\
            - {cd_cmd} {name}\n\
            - {bin} {dev_cmd}",
            title = "What's Next?".bright_white().bold(),
            bin = BIN_NAME.blue(),
            init_cmd = "init [contracts|node|etc...]".blue(),
            build_cmd = "build [contracts|node|etc...]".blue(),
            release_cmd = "release [contracts|node|etc...]".blue(),
            dev_cmd = "dev".blue(),
            deploy_cmd = "deploy [contracts|infra|all] --name <deployment_name>".blue(),
            cd_cmd = "cd".blue(),
            name = name.blue()
        );

        Ok(())
    }
}
