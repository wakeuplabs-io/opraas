use crate::config::get_config_path;
use crate::console::{print_info, print_success, style_spinner};
use clap::ValueEnum;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
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
}

// implementations ================================================

impl InitCommand {
    pub fn new(target: InitTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        let artifacts_factory = ArtifactFactory::new(&project, &config);
        let artifacts = match target {
            InitTargets::All => artifacts_factory.get_all(),
            InitTargets::Batcher => vec![artifacts_factory.get(ArtifactKind::Batcher)],
            InitTargets::Node => vec![artifacts_factory.get(ArtifactKind::Node)],
            InitTargets::Contracts => vec![artifacts_factory.get(ArtifactKind::Contracts)],
            InitTargets::Proposer => vec![artifacts_factory.get(ArtifactKind::Proposer)],
            InitTargets::Geth => vec![artifacts_factory.get(ArtifactKind::Geth)],
        };

        Self { artifacts }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let started = Instant::now();

        print_info("📦 Downloading and preparing artifacts...");

        // Iterate over the artifacts and download
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&ref artifact| {
                let artifact = Arc::new(artifact.clone());

                thread::spawn(move || {
                    match ArtifactInitializer::new().initialize(&artifact) {
                        Ok(_) => println!("✅ {} done", &artifact),
                        Err(e) => {
                            println!("❌ Error setting up {:?}", artifact);
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
