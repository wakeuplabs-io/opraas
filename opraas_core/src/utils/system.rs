use std::{fs, io, path::PathBuf, process::{Command, Stdio}};


pub fn execute_command(command: &mut Command) -> Result<String, String> {
    let output = command.stdout(Stdio::piped()).output().map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        let result =   String::from_utf8_lossy(&output.stdout)
            .to_string();
        Ok(result)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(error_message.to_string())
    }
}

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
