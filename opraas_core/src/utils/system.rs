use log::info;
use std::{
    fs,
    io::{self},
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn execute_command(command: &mut Command, silent: bool) -> Result<String, String> {
    info!("Executing command: {:?}", command);

    if !silent && log::log_enabled!(log::Level::Debug) {
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
    }

    let output = command
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    let status = output.status;

    if status.success() {
        return Ok(result);
    } else {
        return Err(format!("Command exited with non-zero status: {}", status));
    }
}

pub fn copy_and_overwrite(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    if dest.exists() {
        fs::remove_file(dest)?;
    }

    if src.is_file() {
        fs::copy(src, dest)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a file.",
        ));
    }

    Ok(())
}
