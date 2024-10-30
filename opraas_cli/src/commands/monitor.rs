use async_trait::async_trait;
use crate::console;

pub struct MonitorCommand {
    pub target: String,
}

#[async_trait]
impl crate::Runnable for MonitorCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        
        match self.target.as_ref() {
            "contracts" => {
                console::info("Monitoring contracts...");
            },
            "infra" => {
                console::info("Monitoring infra...");
            },
            _ => return Err("Invalid target".into()),
        }

        Ok(())
    }
}

