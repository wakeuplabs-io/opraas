use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;

use super::system;

pub fn clone(
    source_repo: &str,
    source_tag: &str,
    dst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    system::execute_command(
        Command::new("git")
            .arg("clone")
            .arg("--branch")
            .arg(source_tag)
            .arg("--depth")
            .arg("1")
            .arg(format!("https://github.com/{}", source_repo))
            .arg(dst_path),
        true,
    )?;

    Ok(())
}

pub fn download_release_asset(
    release_repo: &str,
    release_tag: &str,
    asset_path: &str,
    dst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(&format!(
        "https://raw.githubusercontent.com/{}/refs/tags/{}/{}",
        release_repo, release_tag, asset_path
    ))?;
    let bytes = response.bytes()?;

    let dst_dir = Path::new(dst_path).parent().unwrap();
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }
    fs::write(dst_path, bytes)?;

    Ok(())
}

pub fn download_zipped_asset(
    release_repo: &str,
    release_tag: &str,
    asset: &str,
    dst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(&format!(
        "https://github.com/{}/releases/download/{}/{}.zip",
        release_repo, release_tag, asset
    ))?;
    let bytes = response.bytes()?;

    let target = Path::new(dst_path);
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    zip_extract::extract(Cursor::new(bytes), &target, true)?;

    Ok(())
}
