use crate::config::{AccountsConfig, DeployConfig, NetworkConfig};
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
    let deploy_cfg_path = Path::new(target.as_ref()).join("deploy-config.json");
    let artifacts_path = Path::new(target.as_ref()).join("artifact.json");

    // generate the deploy config and save it as a file
    let deploy_cfg = DeployConfig::create(accounts_cfg, network_cfg).await;
    File::create(&deploy_cfg_path)
        .unwrap()
        .write_all(
            serde_json::to_string_pretty(&deploy_cfg)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    // TODO: generate salt

    // run the deploy script
    let deploy_out = Command::new("forge")
        .current_dir(source)
        .env("IMPL_SALT", "salt")
        .env("DEPLOY_CONFIG_PATH", deploy_cfg_path.to_str().expect("Invalid UTF-8 path"))
        .env("DEPLOYMENT_OUTFILE", artifacts_path.to_str().expect("Invalid UTF-8 path"))
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
        let error_message = String::from_utf8_lossy(&deploy_out.stderr);
        return Err(format!("Error deploying source: {}", error_message));
    }

    Ok(())
}
