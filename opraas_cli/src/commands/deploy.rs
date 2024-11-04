use async_trait::async_trait;
use log::info;

pub struct  DeployCommand {
    pub target: String,
    pub name: String,
}

impl DeployCommand {
    pub fn new(target: String, name: String) -> Self {
        Self { target, name }
    }
}

#[async_trait]
impl crate::Runnable for DeployCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        cfg.core.as_ref().ok_or_else(|| "Core config not found. Run setup first")?;

        match self.target.as_ref() {
            "contracts" => {
                info!("Deploying contracts for {}...", self.name);
            },
            "infra" => {
                info!("Deploying infra for {}...", self.name);
            },
            "all" => {
                info!("Deploying all artifacts for {}...", self.name);
            }
            _ => return Err("Invalid target".into()),
        }
    
        Ok(())
    }
}

