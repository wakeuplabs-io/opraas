use crate::{
    config::get_config_path,
    console::{print_error, print_info, print_warning, style_spinner, Dialoguer, TDialoguer},
    git::TGit,
};
use clap::ValueEnum;
use colored::*;
use indicatif::{HumanDuration, ProgressBar};
use opraas_core::{
    application::{ArtifactReleaserService, TArtifactReleaserService},
    config::CoreConfig,
    domain::{Artifact, ArtifactFactory, ArtifactKind, Project},
};
use std::{sync::Arc, thread, time::Instant};

pub struct ReleaseCommand {
    git: Box<dyn TGit + Send + Sync>,
    dialoguer: Box<dyn TDialoguer + Send + Sync>,
    artifacts: Vec<Arc<Artifact>>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ReleaseTargets {
    Batcher,
    Node,
    Contracts,
    Proposer,
    Geth,
    All,
}

// implementations ================================================

impl ReleaseCommand {
    pub fn new(target: ReleaseTargets) -> Self {
        let config = CoreConfig::new_from_toml(&get_config_path()).unwrap();
        let project = Project::new_from_root(std::env::current_dir().unwrap());

        let artifacts_factory = ArtifactFactory::new(&project, &config);
        let artifacts = match target {
            ReleaseTargets::All => artifacts_factory.get_all(),
            ReleaseTargets::Batcher => vec![artifacts_factory.get(ArtifactKind::Batcher)],
            ReleaseTargets::Node => vec![artifacts_factory.get(ArtifactKind::Node)],
            ReleaseTargets::Contracts => vec![artifacts_factory.get(ArtifactKind::Contracts)],
            ReleaseTargets::Proposer => vec![artifacts_factory.get(ArtifactKind::Proposer)],
            ReleaseTargets::Geth => vec![artifacts_factory.get(ArtifactKind::Geth)],
        };

        Self {
            git: Box::new(crate::git::Git::new()),
            dialoguer: Box::new(Dialoguer::new()),
            artifacts,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = std::env::current_dir()?;

        // request release name and repository
        print_info("We'll tag your local builds and push them to your repository.");
        print_warning("Make sure you're docker user has push access to the repository");

        let release_name: String = self.dialoguer.prompt("Input release name (e.g. v0.1.0)");
        let registry_url: String = self
            .dialoguer
            .prompt("Input Docker registry url (e.g. dockerhub.io/wakeuplabs) ");

        // Offer option to tag release in git
        if self
            .dialoguer
            .confirm("Would you also like to tag your local git repository?")
        {
            self.git
                .tag_release(&cwd.to_str().unwrap(), &release_name)?;
        }

        // Iterate over the artifacts and release =========================

        let release_spinner = style_spinner(
            ProgressBar::new_spinner(),
            &format!(
                "⏳ Releasing {}...",
                self.artifacts
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );

        let started = Instant::now();
        let handles: Vec<_> = self
            .artifacts
            .iter()
            .map(|&ref artifact| {
                let release_name = release_name.clone();
                let registry_url = registry_url.clone();
                let artifact = Arc::clone(artifact);

                thread::spawn(move || -> Result<(), String> {
                    match ArtifactReleaserService::new().release(
                        &artifact,
                        &release_name,
                        &registry_url,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            print_error(&format!("❌ Error releasing {}", artifact));
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

        release_spinner.finish_with_message(format!(
            "🎉 Released in {}",
            HumanDuration(started.elapsed())
        ));

        // print instructions  =========================

        let bin_name = env!("CARGO_PKG_NAME");

        println!("\n{}\n", "What's Next?".bright_white().bold());

        println!("  {} {}", bin_name.blue(), "dev".blue());
        println!("    Try your artifacts locally without spending any resources.\n");

        println!(
            "  {} {}",
            bin_name.blue(),
            "deploy [contracts|infra|all] --name <deployment_name>".blue()
        );
        println!("    Use your artifacts to create contracts deployments or whole infra.\n");

        Ok(())
    }
}
