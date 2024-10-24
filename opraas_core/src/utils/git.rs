use std::error::Error;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;

pub async fn download_release<P: AsRef<Path>>(
    url: &str,
    destination: &P,
) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if response.status() != 200 {
        return Err("Failed to download release".into());
    }

    let bytes = response.bytes().await?;

    // ensure destination exists
    if !destination.as_ref().exists() {
        std::fs::create_dir_all(destination.as_ref())?;
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

pub fn cherry_pick<P: AsRef<Path>>(repo_path: &P, commit_hash: &str) -> Result<(), String> {
    let output = Command::new("git")
        .arg("cherry-pick")
        .arg(commit_hash)
        .current_dir(repo_path.as_ref())
        .output()
        .expect("Failed to execute git cherry-pick command");

    if !output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(format!("Error cherry-picking commit: {}", error_message))
    }
}
