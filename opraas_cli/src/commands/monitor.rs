use async_trait::async_trait;
use log::info;

pub struct MonitorCommand {
    pub target: String,
}

impl MonitorCommand {
    pub fn new(target: String) -> Self {
        Self { target }
    }
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

