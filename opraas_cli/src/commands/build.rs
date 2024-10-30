use opraas_core::opstack;
use async_trait::async_trait;

pub struct BuildCommand {
    pub target: String,
}

#[async_trait]
impl crate::Runnable for BuildCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Building {}...", self.target);

        match self.target.as_ref() {
            "op-geth" => opstack::geth::build(&cfg.sources.op_geth_repo_target, &self.target)?,
            // "op-proposer" => opstack::proposer::build(&cfg.sources.op_repo_target, &target).unwrap(),
            // "op-batcher" => opstack::batcher::build(&cfg.sources.op_repo_target, &target).unwrap(),
            // "op-node" => opstack::node::build(&cfg.sources.op_repo_target, &target).unwrap(),
            // "op-contracts" => opstack::contracts::build(&cfg.sources.op_repo_target).unwrap(),
            // "all" => {
            //     opstack::geth::build(&cfg.sources.op_geth_repo_target, &target).unwrap();
            //     opstack::proposer::build(&cfg.sources.op_repo_target, &target).unwrap();
            //     opstack::batcher::build(&cfg.sources.op_repo_target, &target).unwrap();
            //     opstack::node::build(&cfg.sources.op_repo_target, &target).unwrap();
            //     opstack::contracts::build(&cfg.sources.op_repo_target).unwrap();
            // }
            _ => panic!("Unknown target: {}", self.target),
        }
    
        Ok(())
    }
    
}
