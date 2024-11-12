use crate::{
    artifacts_factory::{self, ArtifactFactoryTarget},
    config::get_config_path,
    console::{print_info, print_success, style_spinner},
};
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::{
    application::build::{ArtifactBuilderService, TArtifactBuilderService},
    config::CoreConfig,
    domain::{Artifact, Project},
};
use std::{sync::Arc, thread, time::Instant};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BuildTargets {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

impl From<BuildTargets> for ArtifactFactoryTarget {
    fn from(value: BuildTargets) -> Self {
        match value {
            BuildTargets::Batcher => ArtifactFactoryTarget::Batcher,
            BuildTargets::Node => ArtifactFactoryTarget::Node,
            BuildTargets::Contracts => ArtifactFactoryTarget::Contracts,
            BuildTargets::Explorer => ArtifactFactoryTarget::Explorer,
            BuildTargets::Proposer => ArtifactFactoryTarget::Proposer,
            BuildTargets::Geth => ArtifactFactoryTarget::Geth,
            BuildTargets::All => ArtifactFactoryTarget::All,
        }
    }
}

pub struct BuildCommand {
    artifacts: Vec<(&'static str, Arc<Artifact>)>,
}

impl BuildCommand {
    pub fn new(target: BuildTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        Self {
            artifacts: artifacts_factory::create_artifacts(target.into(), &project, &config),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();

        // Iterate over the artifacts and build
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Building {}", name).as_str(),
                );

                thread::spawn(move || -> Result<(), String> {
                    match ArtifactBuilderService::new().build(&artifact) {
                        Ok(_) => spinner.finish_with_message("Waiting..."),
                        Err(e) => {
                            spinner.finish_with_message(format!("❌ Error setting up {}", name));
                            return Err(e.to_string());
                        }
                    }
                    Ok(())
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            match handle.join() {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
                }
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Thread panicked",
                    )))
                }
            }
        }
        m.clear()?;
        print_success(&format!("🎉 Built in {}", HumanDuration(started.elapsed())));
        print_info("Test your build with `opraas dev` and whenever you're ready release `opraas release <name>` and deploy it with `opraas deploy <name>`");

        Ok(())
    }
}
