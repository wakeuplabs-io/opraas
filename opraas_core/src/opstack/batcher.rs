use crate::utils;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build<P: AsRef<Path>, Q: AsRef<Path>>(source: &P, destination: &Q) -> Result<(), String> {
    let build_out = Command::new("make")
        .arg("op-batcher")
        .current_dir(source)
        .output()
        .expect("Failed to execute build command");
    if !build_out.status.success() {
        let error_message = String::from_utf8_lossy(&build_out.stderr);
        return Err(format!("Error building source: {}", error_message));
    }

    // create the destination if it doesn't exist
    if !destination.as_ref().exists() {
        fs::create_dir_all(destination.as_ref()).expect("Failed to create destination directory");
    }

    // get the path to the binaries
    let opt_batcher_bin = Path::new(destination.as_ref()).join("op-batcher");

    // copy the binaries to the desired destination folder
    fs::copy(
        Path::new(source.as_ref()).join("op-batcher/bin/op-batcher"),
        &opt_batcher_bin,
    )
    .expect("Failed to copy opt batcher binary");


    Ok(())
}
