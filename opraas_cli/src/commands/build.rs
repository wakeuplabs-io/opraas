use crate::{
    config::get_config_path,
    console::{print_info, print_success, style_spinner},
};
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::{
    application::build::{ArtifactBuilderService, TArtifactBuilderService},
    config::CoreConfig,
    domain::{Artifact, ArtifactFactory, ArtifactKind, Project},
};
use std::{sync::Arc, thread, time::Instant};

pub struct BuildCommand {
    artifacts: Vec<Arc<Artifact>>,
}

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

// implementations ================================================

impl BuildCommand {
    pub fn new(target: BuildTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        let artifacts_factory = ArtifactFactory::new(&project, &config);
        let artifacts = match target {
            BuildTargets::All => artifacts_factory.get_all(),
            BuildTargets::Batcher => vec![artifacts_factory.get(ArtifactKind::Batcher)],
            BuildTargets::Node => vec![artifacts_factory.get(ArtifactKind::Node)],
            BuildTargets::Contracts => vec![artifacts_factory.get(ArtifactKind::Contracts)],
            BuildTargets::Explorer => vec![artifacts_factory.get(ArtifactKind::Explorer)],
            BuildTargets::Proposer => vec![artifacts_factory.get(ArtifactKind::Proposer)],
            BuildTargets::Geth => vec![artifacts_factory.get(ArtifactKind::Geth)],
        };

        Self { artifacts }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();

        // Iterate over the artifacts and build
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&ref artifact| {
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Building {}", artifact).as_str(),
                );

                thread::spawn(move || -> Result<(), String> {
                    match ArtifactBuilderService::new().build(&artifact) {
                        Ok(_) => spinner.finish_with_message("Waiting..."),
                        Err(e) => {
                            spinner
                                .finish_with_message(format!("❌ Error setting up {:?}", artifact));
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
