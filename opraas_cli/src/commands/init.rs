use crate::config::{SystemRequirementsChecker, TSystemRequirementsChecker, BIN_NAME, GIT_REQUIREMENT};
use crate::console::{print_error, style_spinner};
use clap::ValueEnum;
use colored::*;
use indicatif::{HumanDuration, ProgressBar};
use opraas_core::application::initialize::{ArtifactInitializer, TArtifactInitializerService};
use opraas_core::config::CoreConfig;
use opraas_core::domain::{artifact::Artifact, project::Project};
use opraas_core::domain::{ArtifactFactory, ArtifactKind};
use std::{sync::Arc, thread, time::Instant};

#[derive(Debug, Clone, ValueEnum)]
pub enum InitTargets {
    Batcher,
    Node,
    Contracts,
    Proposer,
    Geth,
    All,
}

pub struct InitCommand {
    artifacts: Vec<Arc<Artifact>>,
    system_requirement_checker: Box<dyn TSystemRequirementsChecker>
}

// implementations ================================================

impl InitCommand {
    pub fn new(target: InitTargets) -> Self {
        let project = Project::new_from_cwd().unwrap();
        let config = CoreConfig::new_from_toml(&project.config).unwrap();

        let artifacts_factory = ArtifactFactory::new(&project, &config);
        let artifacts = match target {
            InitTargets::All => artifacts_factory.get_all(),
            InitTargets::Batcher => vec![artifacts_factory.get(ArtifactKind::Batcher)],
            InitTargets::Node => vec![artifacts_factory.get(ArtifactKind::Node)],
            InitTargets::Contracts => vec![artifacts_factory.get(ArtifactKind::Contracts)],
            InitTargets::Proposer => vec![artifacts_factory.get(ArtifactKind::Proposer)],
            InitTargets::Geth => vec![artifacts_factory.get(ArtifactKind::Geth)],
        };

        Self { artifacts, system_requirement_checker: Box::new(SystemRequirementsChecker::new()) }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.system_requirement_checker.check(vec![
            GIT_REQUIREMENT,
        ])?;

        // start timer and spinner
        let started = Instant::now();
        let init_spinner = style_spinner(
            ProgressBar::new_spinner(),
            &format!(
                "⏳ Initializing {}...",
                self.artifacts
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );

        // iterate over the artifacts and download
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&ref artifact| {
                let artifact = Arc::new(artifact.clone());

                thread::spawn(move || {
                    match ArtifactInitializer::new().initialize(&artifact) {
                        Ok(_) => {}
                        Err(e) => {
                            print_error(&format!("❌ Error initializing {}", artifact));
                            return Err(e.to_string());
                        }
                    }
                    Ok(())
                })
            })
            .collect();

        // wait for all threads to complete
        for handle in handles {
            match handle.join() {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))),
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Thread panicked",
                    )))
                }
            }
        }

        init_spinner.finish_with_message(format!("Done in {}", HumanDuration(started.elapsed())));

        // print instructions ========================================

        println!(
            "\n{title}\n\n\
            - {bin} {build_cmd}\n\
            \tBuilds docker images from artifacts.\n\n\
            - {bin} {release_cmd}\n\
            \tPublishes docker images to be used in dev or prod.\n\n\
            - {bin} {dev_cmd}\n\
            \tRuns a local dev environment.\n\n\
            - {bin} {deploy_cmd}\n\
            \tDeploys contracts to l1 and infra to kubernetes through terraform.\n",
            title = "What's Next?".bright_white().bold(),
            bin = BIN_NAME.blue(),
            build_cmd = "build [contracts|node|etc...]".blue(),
            release_cmd = "release [contracts|node|etc...]".blue(),
            dev_cmd = "dev".blue(),
            deploy_cmd = "deploy [contracts|infra|all] --name <deployment_name>".blue()
        );

        Ok(())
    }
}
