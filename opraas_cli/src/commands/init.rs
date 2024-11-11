use crate::console::{print_info, print_success, style_spinner};
use clap::ValueEnum;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::application::initialize_artifact::{ArtifactInitializer, TArtifactInitializerService};
use opraas_core::config::{Config, CoreConfig};
use opraas_core::domain::{artifact::Artifact, project::Project};
use std::{sync::Arc, thread, time::Instant};

pub struct InitCommand<'a> {
    artifacts: Vec<(&'static str, Arc<Artifact<'a>>)>,
}

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

impl<'a> InitCommand<'a> {
    pub fn new(target: InitTargets, project: &Project, cfg: &'a CoreConfig) -> Self {
        let mut artifacts: Vec<(&'static str, Arc<Artifact<'a>>)> = vec![
            (
                "Batcher",
                Arc::new(Artifact::new(&project.src.batcher, &cfg.artifacts.batcher)),
            ),
            (
                "Node",
                Arc::new(Artifact::new(&project.src.node, &cfg.artifacts.node)),
            ),
            (
                "Contracts",
                Arc::new(Artifact::new(&project.src.contracts, &cfg.artifacts.contracts)),
            ),
            (
                "Explorer",
                Arc::new(Artifact::new(&project.src.explorer, &cfg.artifacts.explorer)),
            ),
            (
                "Proposer",
                Arc::new(Artifact::new(&project.src.proposer, &cfg.artifacts.proposer)),
            ),
            (
                "Geth",
                Arc::new(Artifact::new(&project.src.geth, &cfg.artifacts.geth)),
            ),
        ];

        artifacts.retain(|(name, _)| match target {
            InitTargets::Batcher => *name == "Batcher",
            InitTargets::Node => *name == "Node",
            InitTargets::Contracts => *name == "Contracts",
            InitTargets::Explorer => *name == "Explorer",
            InitTargets::Proposer => *name == "Proposer",
            InitTargets::Geth => *name == "Geth",
            _ => false,
        });

        Self { artifacts }
    }

    pub fn run(&'static self) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();

        print_info("📦 Downloading and preparing artifacts...");

        // Iterate over the artifacts and download
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
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
