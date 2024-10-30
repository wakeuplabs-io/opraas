use async_trait::async_trait;
use opraas_core::opstack;

pub struct SetupCommand;

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Setting up project");

        let cwd = std::env::current_dir().expect("Failed to get current directory");
        let op_target = cwd.join(&cfg.sources.op_repo_target);
        let op_geth_target = cwd.join(&cfg.sources.op_geth_repo_target);
        let bin_target = cwd.join("bin");

        println!("Downloading source...");
        opstack::source::download(
            &cfg.sources.op_repo_url,
            &cfg.sources.op_repo_tag,
            &cfg.sources.op_repo_target,
            &cfg.sources.op_geth_repo_url,
            &cfg.sources.op_geth_repo_tag,
            &cfg.sources.op_geth_repo_target,
        )
        .expect("Failed to download optimism source");

        println!("Building optimism batcher...");
        opstack::batcher::build(&op_target, &bin_target).expect("Failed to build optimism batcher");

        println!("Building optimism node...");
        opstack::node::build(&op_target, &bin_target).expect("Failed to build optimism node");

        println!("Building optimism proposer...");
        opstack::proposer::build(&op_target, &bin_target)
            .expect("Failed to build optimism proposer");

        println!("Building optimism contracts...");
        opstack::contracts::build(&op_target).expect("Failed to build optimism contracts");

        println!("Building op-geth...");
        opstack::geth::build(&op_geth_target, &bin_target).expect("Failed to build op-geth");

        Ok(())
    }
}
