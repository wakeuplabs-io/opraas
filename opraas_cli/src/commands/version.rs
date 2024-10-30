use crate::Runnable;
use crate::console;
use async_trait::async_trait;

pub struct VersionCommand;

#[async_trait]
impl Runnable for VersionCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        console::info(format!("{}", env!("CARGO_PKG_VERSION")));
        
        Ok(())
    }
}
