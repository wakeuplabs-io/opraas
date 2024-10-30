use async_trait::async_trait;

pub struct  DeployCommand {
    pub target: String,
    pub name: String,
}

#[async_trait]
impl crate::Runnable for DeployCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
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

