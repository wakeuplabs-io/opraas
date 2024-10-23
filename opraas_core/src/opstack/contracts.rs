use crate::config::{AccountsConfig, DeployConfig, NetworkConfig};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn build<P: AsRef<Path>>(source: &P) -> Result<(), String> {
    let build_out = Command::new("make")
        .arg("build-contracts")
        .current_dir(source)
        .output()
        .expect("Failed to execute build command");
    if !build_out.status.success() {
        let error_message = String::from_utf8_lossy(&build_out.stderr);
        return Err(format!("Error building source: {}", error_message));
    }

    Ok(())
}

pub async fn deploy<P: AsRef<Path>, Q: AsRef<Path>>(
    source: &P,
    target: &Q,
    network_cfg: &NetworkConfig,
    accounts_cfg: &AccountsConfig,
) -> Result<(), String> {
    // ensure we're not overwriting an existing deployment
    if target.as_ref().exists() {
        return Err(String::from("Deployment already exists"));
    }
    fs::create_dir_all(target.as_ref()).expect("Failed to create target directory");

    // create paths, in is inside the bedrock directory and what the scripts consume. out is a copy to the target
    let contracts_source = Path::new(source.as_ref()).join("packages/contracts-bedrock");
    let deploy_cfg_path_in = contracts_source
        .clone()
        .join("deploy-config/opraas-config.json");
    let artifacts_path_in = contracts_source
        .clone()
        .join("deployments/opraas-artifacts.json");
    let deploy_cfg_path_out = Path::new(target.as_ref()).join("deploy-config.json");
    let artifacts_path_out = Path::new(target.as_ref()).join("artifact.json");

    // generate the deploy config and save it as a file the script can read
    let deploy_cfg = DeployConfig::create(accounts_cfg, network_cfg).await;
    File::create(&deploy_cfg_path_in)
        .unwrap()
        .write_all(
            serde_json::to_string_pretty(&deploy_cfg)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    fs::copy(&deploy_cfg_path_in, &deploy_cfg_path_out).expect("Failed to copy deploy config");

    let deploy_out = Command::new("forge")
        .current_dir(contracts_source)
        .env("IMPL_SALT", uuid::Uuid::new_v4().to_string())
        .env(
            "DEPLOY_CONFIG_PATH",
            deploy_cfg_path_in.to_str().expect("Invalid UTF-8 path"),
        )
        .env(
            "DEPLOYMENT_OUTFILE",
            artifacts_path_in.to_str().expect("Invalid UTF-8 path"),
        )
        .arg("script")
        .arg("scripts/deploy/Deploy.s.sol:Deploy")
        .arg("--broadcast")
        .arg("--private-key")
        .arg(accounts_cfg.deployer_private_key.clone())
        .arg("--rpc-url")
        .arg(network_cfg.l1_rpc_url.clone())
        .output()
        .expect("Failed to execute deploy command");

    if !deploy_out.status.success() {
        println!(
            "Error deploying source: {}",
            String::from_utf8_lossy(&deploy_out.stdout)
        );
        let error_message = String::from_utf8_lossy(&deploy_out.stderr);
        return Err(format!("Error deploying source: {}", error_message));
    }

    fs::copy(&artifacts_path_in, &artifacts_path_out)
        .expect("Failed to copy deploy artifacts file");

    Ok(())
}
