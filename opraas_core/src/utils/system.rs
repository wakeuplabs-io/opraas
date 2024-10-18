use std::fs::{self};
use std::path::Path;
use std::io;
use std::os::unix::fs::PermissionsExt;

pub fn set_file_permissions<P: AsRef<Path>>(file_path: P, mode: u32) -> Result<(), io::Error> {
    let mut permissions = fs::metadata(&file_path)?.permissions();
    permissions.set_mode(mode);
    
    fs::set_permissions(&file_path, permissions)?;
    
    Ok(())
}
