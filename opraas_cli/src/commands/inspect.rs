use async_trait::async_trait;

pub struct InspectCommand {
    pub target: String
}

#[async_trait]
impl crate::Runnable for InspectCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

