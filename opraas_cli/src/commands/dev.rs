use async_trait::async_trait;

pub struct DevCommand;

#[async_trait]
impl crate::Runnable for DevCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        cfg.core.as_ref().ok_or_else(|| "Core config not found. Run setup first")?;

        info!("Starting dev environment...");

        Ok(())
    }
}

