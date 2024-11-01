use async_trait::async_trait;
use log::info;

pub struct DevCommand;

#[async_trait]
impl crate::Runnable for DevCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        cfg.core.as_ref().ok_or_else(|| "Core config not found. Create project with opraas new")?;

        info!("Starting dev environment...");

        Ok(())
    }
}
