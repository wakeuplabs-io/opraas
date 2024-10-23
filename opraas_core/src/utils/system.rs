use std::fs::{self};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn set_file_permissions<P: AsRef<Path>>(file_path: P, mode: u32) -> Result<(), io::Error> {
    let mut permissions = fs::metadata(&file_path)?.permissions();
    permissions.set_mode(mode);

    fs::set_permissions(&file_path, permissions)?;

    Ok(())
}
