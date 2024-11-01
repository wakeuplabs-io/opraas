use assert_cmd::prelude::*;
use std::process::Command;

const BIN: &str = env!("CARGO_PKG_NAME");

#[test]
fn create_new_project() {
    let new_dir = "my-chain-a";

    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("new").arg(new_dir).assert().success();

    // check if directory exists
    let new_dir_path = std::env::current_dir().unwrap().join(new_dir);
    assert!(new_dir_path.exists());

    // check if README.md exists
    let readme_path = new_dir_path.join("README.md");
    assert!(readme_path.exists());

    // check if gitignore exists
    let gitignore_path = new_dir_path.join(".gitignore");
    assert!(gitignore_path.exists());

    // check if config.toml exists
    let config_path = new_dir_path.join("config.toml");
    assert!(config_path.exists());

    // cleanup
    std::fs::remove_dir_all(new_dir_path).unwrap();
}


#[test]
fn create_new_project_fails_if_dir_exists() {
    let new_dir = "my-chain";

    // create directory
    let new_dir_path = std::env::current_dir().unwrap().join(new_dir);
    std::fs::create_dir(&new_dir_path).unwrap();

    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("new").arg(new_dir).assert().failure().stderr(predicates::str::contains("Directory already exists"));

    // cleanup
    std::fs::remove_dir_all(new_dir_path).unwrap();
}

fn get_tests_dir() -> std::path::PathBuf {
    // get tmp dir
    let path = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));

    // ensure tests dir exists
    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path
}
