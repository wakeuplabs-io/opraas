use assert_cmd::prelude::*;
use std::{fs, path::PathBuf, process::Command};

const BIN: &str = env!("CARGO_PKG_NAME");
#[test]
fn create_new_project() {
    let new_dir = get_tests_dir().join("create_new_project");

    // Ensure the directory doesn't exist before starting
    if new_dir.exists() {
        fs::remove_dir_all(&new_dir).unwrap();
    }

    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("new").arg(&new_dir).assert().success();

    // Assert that files have been created
    assert!(new_dir.exists(), "New directory was not created");
    assert!(new_dir.join("README.md").exists(), "README.md was not created");
    assert!(new_dir.join(".gitignore").exists(), ".gitignore was not created");
    assert!(new_dir.join("config.toml").exists(), "config.toml was not created");
    assert!(new_dir.join(".env").exists(), ".env was not created");
    
    // Assert git was initialized
    assert!(new_dir.join(".git").exists(), ".git was not created");

    // Cleanup
    fs::remove_dir_all(new_dir).unwrap();
}

#[test]
fn create_new_project_fails_if_dir_exists() {
    let new_dir = get_tests_dir().join("create_new_project_fails_if_dir_exists");
    
    // Create the directory before running the command
    if !new_dir.exists() {
        fs::create_dir(&new_dir).unwrap();
    }

    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("new").arg(&new_dir)
        .assert()
        .failure()
        .stderr(predicates::str::contains("Directory already exists"));

    // Cleanup
    fs::remove_dir_all(new_dir).unwrap();
}

fn get_tests_dir() -> PathBuf {
    let path = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));
    
    // Ensure the test directory is clean
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    path
}