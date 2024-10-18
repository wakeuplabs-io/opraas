use crate::config::config::Config;
use opraas_core::opstack;

pub async fn deploy(cfg: &Config, target: &str) {
    println!("Deploying {}...", target);

    match target {
        "contracts" => opstack::contracts::deploy(
            &cfg.sources.op_repo_target,
            &cfg.sources.op_repo_target, // TODO:
            &cfg.network,
            &cfg.accounts,
        )
        .await
        .unwrap(),
        _ => panic!("Unknown target: {}", target),
    }
}
