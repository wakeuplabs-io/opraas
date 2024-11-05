use log::info;
use async_trait::async_trait;
use clap::ValueEnum;

pub struct InspectCommand {
     target: InspectTarget
}

#[derive(Debug, Clone, ValueEnum)]
pub enum InspectTarget {
    Contracts,
    Infra
}

impl InspectCommand {
    pub fn new(target: InspectTarget) -> Self {
        Self { target }
    }
}

#[async_trait]
impl crate::Runnable for InspectCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {

        match self.target {
            InspectTarget::Contracts => {
                info!("Inspecting contracts");
            },
            InspectTarget::Infra => {
                info!("Inspecting infra");
            }
        }

        Ok(())
    }
}

