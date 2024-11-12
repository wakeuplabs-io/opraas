use crate::{
    artifacts_factory::{self, ArtifactFactoryTarget},
    config::get_config_path,
    console::{
        print_error, print_info, print_success, print_warning, style_spinner, Dialoguer, TDialoguer,
    },
    git::TGit,
};
use clap::ValueEnum;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use opraas_core::{
    application::{ArtifactReleaserService, TArtifactReleaserService},
    config::CoreConfig,
    domain::{Artifact, Project},
};
use std::{sync::Arc, thread, time::Instant};

#[derive(Debug, Clone, ValueEnum)]
pub enum ReleaseTargets {
    Batcher,
    Node,
    Contracts,
    Explorer,
    Proposer,
    Geth,
    All,
}

impl From<ReleaseTargets> for ArtifactFactoryTarget {
    fn from(value: ReleaseTargets) -> Self {
        match value {
            ReleaseTargets::Batcher => ArtifactFactoryTarget::Batcher,
            ReleaseTargets::Node => ArtifactFactoryTarget::Node,
            ReleaseTargets::Contracts => ArtifactFactoryTarget::Contracts,
            ReleaseTargets::Explorer => ArtifactFactoryTarget::Explorer,
            ReleaseTargets::Proposer => ArtifactFactoryTarget::Proposer,
            ReleaseTargets::Geth => ArtifactFactoryTarget::Geth,
            ReleaseTargets::All => ArtifactFactoryTarget::All,
        }
    }
}

pub struct ReleaseCommand {
    git: Box<dyn TGit + Send + Sync>,
    dialoguer: Box<dyn TDialoguer + Send + Sync>,
    artifacts: Vec<(&'static str, Arc<Artifact>)>,
}

impl ReleaseCommand {
    pub fn new(target: ReleaseTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        Self {
            git: Box::new(crate::git::Git::new()),
            dialoguer: Box::new(Dialoguer::new()),
            artifacts: artifacts_factory::create_artifacts(target.into(), &project, &config),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = std::env::current_dir()?;

        // avoid releasing without committed changes
        if self.git.has_uncommitted_changes(cwd.to_str().unwrap()) {
            print_error("You have uncommitted changes. Please commit first.");
            return Ok(());
        }

        // request release name and repository
        print_info("We'll tag your local builds and push them to your repository.");
        print_warning("Make sure you're docker user has push access to the repository");

        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let release_repository: String = self
            .dialoguer
            .prompt("Input Docker repository url (e.g. docker.io/wakeuplabs) ");

        // Offer option to tag release in git
        if self
            .dialoguer
            .confirm("Would you also like to tag your local git repository?")
        {
            self.git
                .tag_release(&cwd.to_str().unwrap(), &release_name)?;
        }

        // Iterate over the artifacts and build
        let started = Instant::now();
        let m = MultiProgress::new();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&(name, ref artifact)| {
                let release_name = release_name.clone();
                let release_repository = release_repository.clone();
                let artifact = Arc::clone(artifact); // Clone the Arc for thread ownership
                let spinner = style_spinner(
                    m.add(ProgressBar::new_spinner()),
                    format!("⏳ Releasing {}", name).as_str(),
                );

                thread::spawn(move || -> Result<(), String> {
                    match ArtifactReleaserService::new().release(
                        &artifact,
                        &release_name,
                        &release_repository,
                    ) {
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
        print_success(&format!(
            "🎉 Released in {}",
            HumanDuration(started.elapsed())
        ));
        print_info("Yay! Your chain is ready to be deployed!");

        Ok(())
    }
}
