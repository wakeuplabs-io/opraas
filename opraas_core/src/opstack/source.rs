use crate::utils;
use std::path::Path;
use std::process::Command;

pub fn download<P: AsRef<Path>>(
    opt_url: &str, 
    opt_tag: &str, 
    opt_dst: &P,
    geth_url: &str, 
    geth_tag: &str, 
    geth_dst: &P
) -> Result<(), String> {
    // clone optimism repo and update submodules
    utils::git::clone_repo_at_tag(opt_url, opt_tag, opt_dst)?;
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
    utils::git::clone_repo_at_tag(geth_url, geth_tag, geth_dst)?;

    Ok(())
}