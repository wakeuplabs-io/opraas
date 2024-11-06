use async_trait::async_trait;
use log::info;
use clap::ValueEnum;

pub struct MonitorCommand {
    target: MonitorTarget,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum MonitorTarget {
    Contracts,
    Infra
}

impl MonitorCommand {
    pub fn new(target: MonitorTarget) -> Self {
        Self { target }
    }
}

#[async_trait]
impl crate::Runnable for MonitorCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        match self.target {
            MonitorTarget::Contracts => {
                info!("Monitoring contracts");
            },
            MonitorTarget::Infra => {
                info!("Monitoring infra");
            },
        }

        Ok(())
    }
}

