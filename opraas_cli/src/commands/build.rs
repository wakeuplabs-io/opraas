use crate::config::config::Config;
use opraas_core::opstack;

pub fn build(cfg: &Config, target: &str) {
    println!("Building {}...", target);

    match target {
        "op-geth" => opstack::geth::build(&cfg.sources.op_geth_repo_target, &target).unwrap(),
        "op-proposer" => opstack::proposer::build(&cfg.sources.op_repo_target, &target).unwrap(),
        "op-batcher" => opstack::batcher::build(&cfg.sources.op_repo_target, &target).unwrap(),
        "op-node" => opstack::node::build(&cfg.sources.op_repo_target, &target).unwrap(),
        "op-contracts" => opstack::contracts::build(&cfg.sources.op_repo_target).unwrap(),
        "all" => {
            opstack::geth::build(&cfg.sources.op_geth_repo_target, &target).unwrap();
            opstack::proposer::build(&cfg.sources.op_repo_target, &target).unwrap();
            opstack::batcher::build(&cfg.sources.op_repo_target, &target).unwrap();
            opstack::node::build(&cfg.sources.op_repo_target, &target).unwrap();
            opstack::contracts::build(&cfg.sources.op_repo_target).unwrap();
        }
        _ => panic!("Unknown target: {}", target),
    }
}
