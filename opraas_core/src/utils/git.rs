use std::fs;
use std::io::Cursor;
use std::path::Path;

pub fn download_release(
    release_url: &str,
    release_tag: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    download_and_extract(&format!("{}/archive/refs/tags/{}.zip", release_url, release_tag), destination)?;

    Ok(())
}

pub fn download_release_zipped_asset(
    release_url: &str,
    release_tag: &str,
    zip_name: &str,
    destination: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    download_and_extract(&format!("{}/releases/download/{}/{}.zip", release_url, release_tag, zip_name), destination)?;

    Ok(())
}

fn download_and_extract(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;

    let bytes = response.bytes()?;

    let target = Path::new(destination);
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    zip_extract::extract(Cursor::new(bytes), &target, true)?;

    Ok(())
}
