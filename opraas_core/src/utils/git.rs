use std::fs;
use std::io::Cursor;
use std::path::Path;

pub fn download_release(
    release_repo: &str,
    release_tag: &str,
    dst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    download_and_extract(
        &format!("https://github.com/{}/archive/refs/tags/{}.zip", release_repo, release_tag),
        dst_path,
    )?;

    Ok(())
}

pub fn download_release_asset(
    release_repo: &str,
    release_tag: &str,
    asset_path: &str,
    dst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(&format!("https://raw.githubusercontent.com/{}/refs/tags/{}/{}", release_repo, release_tag, asset_path))?;
    let bytes = response.bytes()?;

    let dst_dir = Path::new(dst_path).parent().unwrap();
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }
    fs::write(dst_path, bytes)?;

    Ok(())
}

pub fn download_release_zipped_asset(
    release_url: &str,
    release_tag: &str,
    zip_name: &str,
    dst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    download_and_extract(
        &format!(
            "{}/releases/download/{}/{}.zip",
            release_url, release_tag, zip_name
        ),
        dst_path,
    )?;

    Ok(())
}

fn download_and_extract(url: &str, dst_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    let target = Path::new(dst_path);
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    zip_extract::extract(Cursor::new(bytes), &target, true)?;

    Ok(())
}
