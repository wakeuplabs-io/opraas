use async_trait::async_trait;

pub struct MonitorCommand {
    pub target: String,
}

#[async_trait]
impl crate::Runnable for MonitorCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        
        match self.target.as_ref() {
            "contracts" => {
                info!("Monitoring contracts...");
            },
            "infra" => {
                info!("Monitoring infra...");
            },
            _ => return Err("Invalid target".into()),
        }

        Ok(())
    }
}

