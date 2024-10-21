use crate::config::config::Config;
use opraas_core::opstack;
use std::env;

pub async fn deploy(cfg: &Config, target: &str, name: &str) {
    println!("Deploying {}...", target);

    let cwd = env::current_dir().expect("Failed to get current working directory");
    let target_folder = cwd.join("deployments").join(name);

    match target {
        "contracts" => opstack::contracts::deploy(
            &cfg.sources.op_repo_target,
            &target_folder, 
            &cfg.network,
            &cfg.accounts,
        )
        .await
        .unwrap(),
        _ => panic!("Unknown target: {}", target),
    }

    println!("Successfully deployed. Find assets at: {}", target_folder.to_str().unwrap());
}

