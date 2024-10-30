use async_trait::async_trait;

pub struct DevCommand;

#[async_trait]
impl crate::Runnable for DevCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting dev environment...");

        Ok(())
    }
}

