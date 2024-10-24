use crate::config::sources::Source;
use crate::utils;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

const OPT_NODE_SRC: &str = "src/op_node";
const OPT_NODE_BIN: &str = "bin";

pub async fn download(source: &Source) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://github.com/ethereum-optimism/optimism/archive/refs/tags/{}.zip",
        source.release_tag
    );

    utils::git::download_release(&url, &OPT_NODE_SRC).await?;

    Ok(())
}

pub fn build(source: &Source) -> Result<(), String> {
    // build the binaries
    for cmd in &source.build {
        let output = Command::new("sh")
            .arg("-c") // Run the command through the shell
            .arg(cmd)
            .current_dir(OPT_NODE_SRC)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Error building source: {}", error_message));
        }
    }

    // create the destination if it doesn't exist
    let bin_dest = Path::new(OPT_NODE_BIN);
    if !bin_dest.exists() {
        fs::create_dir_all(OPT_NODE_BIN).expect("Failed to create destination directory");
    }

    // get the path to the binaries
    let opt_node_bin = Path::new(OPT_NODE_BIN).join("op-node");

    // copy the binaries to the desired destination folder
    fs::copy(
        Path::new(OPT_NODE_SRC).join("op-node/bin/op-node"),
        &opt_node_bin,
    )
    .expect("Failed to copy opt node binary");

    // set permissions for execution
    utils::system::set_file_permissions(&opt_node_bin, 0o755)
        .expect("Failed to set node execution permissions");

    Ok(())
}
