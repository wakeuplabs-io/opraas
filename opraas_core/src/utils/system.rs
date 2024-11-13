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
    // Resolve absolute paths to ensure src and dest are not the same
    let src_path = fs::canonicalize(src)?;
    let dest_path = fs::canonicalize(dest)?;

    if src_path == dest_path {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source and destination are the same.",
        ));
    }

    if src_path.is_file() {
        // Copy single file
        fs::copy(&src_path, &dest_path)?;
    } else if src_path.is_dir() {
        // Copy directory contents recursively
        copy_dir_recursive(&src_path, &dest_path)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Source is neither file nor directory."));
    }

    Ok(())
}

/// Recursively copy contents of a directory.
fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}