use async_trait::async_trait;

pub struct SetupCommand;

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        // if no core config exists, create one, prompting for user input
        if (cfg.core).is_none() {
            info!("No core config found. Creating one...");
        }

        Ok(())
    }
}
