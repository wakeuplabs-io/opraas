use crate::Runnable;
use async_trait::async_trait;
use colored::Colorize;

pub struct VersionCommand;

#[async_trait]
impl Runnable for VersionCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Version: {}", env!("CARGO_PKG_VERSION").bold());
        
        Ok(())
    }
}
