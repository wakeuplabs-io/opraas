use async_trait::async_trait;

pub struct BuildCommand {
    pub target: String,
}

#[async_trait]
impl crate::Runnable for BuildCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        match self.target.as_ref() {
            "op-geth" => {
                info!("Building op-geth...");
            },
            "op-proposer" => {
                info!("Building op-proposer...");
            },
            "op-batcher" => {
                info!("Building op-batcher...");
            },
            "op-node" => {
                info!("Building op-node...");
            },
            "op-contracts" => {
                info!("Building op-contracts...");
            },
            "all" => {
                info!("Building all artifacts...");
            }
            _ => return Err("Invalid target".into()),
        }
   
        Ok(())
    }
}
