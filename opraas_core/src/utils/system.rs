use std::{fs, io, path::{Path, PathBuf}, process::Command};


pub fn execute_command(command: &mut Command) -> Result<String, String> {
    let output = command.output().map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        // Convert the output to a String
        let result =   String::from_utf8_lossy(&output.stdout)
            .to_string();
        Ok(result)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(error_message.to_string())
    }
}

/// Copy a file or directory, overwriting if the destination already exists.
pub fn copy_and_overwrite(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    if dest.exists() {
        fs::remove_file(dest)?;
    }

    if src.is_file() {
        fs::copy(src, dest)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Source is not a file."));
    }

    Ok(())
}
