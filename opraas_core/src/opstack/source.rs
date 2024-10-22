use crate::utils;
use std::path::Path;
use std::process::Command;

const OPT_URL: &str = "https://github.com/ethereum-optimism/optimism";
const OPT_TAG: &str = "op-node/v1.3.1";
const OPT_FIXES: &[&str] = &["2e57472890f9fea39cde72537935393b068d3e0f", "5252c82f607af81f6cb741a370425eaf26280892"];
const GETH_URL: &str = "https://github.com/ethereum-optimism/op-geth.git";
const GETH_TAG: &str = "v1.101315.3";

pub fn download<P: AsRef<Path>>(
    opt_dst: &P,
    geth_dst: &P
) -> Result<(), String> {
    // clone optimism repo and update submodules
    utils::git::clone_repo_at_tag(OPT_URL, OPT_TAG, opt_dst)?;
   
    // apply fixes
    for fix in OPT_FIXES {
        utils::git::cherry_pick(opt_dst, fix)?;
    }

    // update submodules
    let build_out = Command::new("make")
        .arg("submodules")
        .current_dir(opt_dst)
        .output()
        .expect("Failed to update submodules");
    if !build_out.status.success() {
        let error_message = String::from_utf8_lossy(&build_out.stderr);
        return Err(format!("Error building source: {}", error_message));
    }

    // clone geth repo
    utils::git::clone_repo_at_tag(GETH_URL, GETH_TAG, geth_dst)?;

    Ok(())
}