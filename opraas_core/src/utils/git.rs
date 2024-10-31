use std::io::Cursor;
use std::path::Path;
use std::process::Command;
use web3::futures::TryStreamExt;

use crate::progress::ProgressTracker;

pub async fn download_release<P: AsRef<Path>, T: ProgressTracker>(
    git_url: &str,
    release: &str,
    destination: &P,
    progress: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(format!("{}/archive/refs/tags/{}.zip", git_url, release)).await?;
    if response.status() != 200 {
        return Err("Failed to download release".into());
    }

    // estimate download size
    let total_size = response.content_length().unwrap_or(0);
    progress.set_length(total_size);

    // ensure destination exists
    if !destination.as_ref().exists() {
        std::fs::create_dir_all(destination.as_ref())?;
    }
    
    // Stream the response bytes and collect them into the vector
    let mut bytes = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.try_next().await? {
        bytes.extend_from_slice(&chunk);

        // update progress as we go
        progress.inc(bytes.len() as u64);
    }

    let target = Path::new(destination.as_ref());
    zip_extract::extract(Cursor::new(bytes), &target, true)?;

    Ok(())
}

pub fn clone_repo_at_tag<P: AsRef<Path>>(
    repo_url: &str,
    branch: &str,
    destination: &P,
) -> Result<(), String> {
    let output = Command::new("git")
        .arg("clone")
        .arg("--branch")
        .arg(branch)
        .arg("--single-branch")
        .arg(repo_url)
        .arg(destination.as_ref())
        .output()
        .expect("Failed to execute git clone command");

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(format!("Error cloning repository: {}", error_message))
    }
}
