use async_trait::async_trait;
use clap::ValueEnum;
use log::info;

pub struct  DeployCommand {
    pub name: String,
    pub target: DeployTarget,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum DeployTarget {
    Contracts,
    Infra,
    All
}

impl DeployCommand {
    pub fn new(target: DeployTarget, name: String) -> Self {
        Self { target, name }
    }
}

#[async_trait]
impl crate::Runnable for DeployCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        cfg.core.as_ref().ok_or_else(|| "Core config not found. Run setup first")?;

        match self.target {
            DeployTarget::Contracts => {
                info!("Deploying contracts");
            },
            DeployTarget::Infra => {
                info!("Deploying infra");
            },
            DeployTarget::All => {
                info!("Deploying all");
            }
        }

        Ok(())
    }
}

