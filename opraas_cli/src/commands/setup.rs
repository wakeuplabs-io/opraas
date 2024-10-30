use async_trait::async_trait;

pub struct SetupCommand;

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
       
        Ok(())
    }
}
