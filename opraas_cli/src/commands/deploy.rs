use opraas_core::opstack;
use async_trait::async_trait;

pub struct  DeployCommand {
    pub target: String,
    pub name: String,
}

#[async_trait]
impl crate::Runnable for DeployCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Deploying {}...", self.target);

        let cwd = std::env::current_dir()?;
        let target_folder = cwd.join("deployments").join(self.name.as_str());
        let source_folder = cwd.join(&cfg.sources.op_repo_target);
    
        match self.target.as_ref() {
            "contracts" => {
                opstack::contracts::deploy(&source_folder, &target_folder, &cfg.network, &cfg.accounts)
                    .await?;
            }
            _ => panic!("Unknown target: {}", self.target),
        }
    
        println!(
            "Successfully deployed. Find assets at: {}",
            target_folder.to_str().unwrap_or(".")
        );
    
        Ok(())
    }
}

