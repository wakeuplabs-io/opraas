use crate::artifacts_factory::{self, ArtifactFactoryTarget};
use crate::config::get_config_path;
use crate::console::{print_info, print_success, style_spinner};
use clap::ValueEnum;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::application::initialize::{ArtifactInitializer, TArtifactInitializerService};
use opraas_core::config::CoreConfig;
use opraas_core::domain::{artifact::Artifact, project::Project};
use std::{sync::Arc, thread, time::Instant};

#[derive(Debug, Clone, ValueEnum)]
pub enum InitTargets {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

impl From<InitTargets> for ArtifactFactoryTarget {
    fn from(value: InitTargets) -> Self {
        match value {
            InitTargets::Batcher => ArtifactFactoryTarget::Batcher,
            InitTargets::Node => ArtifactFactoryTarget::Node,
            InitTargets::Contracts => ArtifactFactoryTarget::Contracts,
            InitTargets::Explorer => ArtifactFactoryTarget::Explorer,
            InitTargets::Proposer => ArtifactFactoryTarget::Proposer,
            InitTargets::Geth => ArtifactFactoryTarget::Geth,
            InitTargets::All => ArtifactFactoryTarget::All,
        }
    }
}

pub struct InitCommand {
    artifacts: Vec<(&'static str, Arc<Artifact>)>,
}

impl InitCommand {
    pub fn new(target: InitTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        Self {
            artifacts: artifacts_factory::create_artifacts(target.into(), &project, &config),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();

        print_info("📦 Downloading and preparing artifacts...");

        // Iterate over the artifacts and download
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let artifact = Arc::new(artifact.clone());
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Preparing {}", name).as_str(),
                );

                thread::spawn(move || {
                    match ArtifactInitializer::new().initialize(&artifact) {
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

        print_success(&format!("🎉 Done in {}", HumanDuration(started.elapsed())));

        Ok(())
    }
}
