use crate::Runnable;
use async_trait::async_trait;

pub struct VersionCommand;

#[async_trait]
impl Runnable for VersionCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
