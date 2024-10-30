use async_trait::async_trait;
use crate::console;

pub struct DevCommand;

#[async_trait]
impl crate::Runnable for DevCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {

        console::info("Starting dev environment...");

        Ok(())
    }
}

