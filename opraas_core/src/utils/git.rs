use std::process::Command;
use std::path::Path;

pub fn clone_repo_at_tag<P: AsRef<Path>>(repo_url: &str, branch: &str, destination: &P) -> Result<(), String> {
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
        Err(format!(
            "Error cloning repository: {}",
            error_message
        ))
    }
}
