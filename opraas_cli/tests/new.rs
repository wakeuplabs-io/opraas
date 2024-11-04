use assert_cmd::prelude::*;
use std::process::Command;

const BIN: &str = env!("CARGO_PKG_NAME");

#[test]
fn create_new_project() {
    let new_dir = get_tests_dir().join("create_new_project");

    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("new").arg(&new_dir).assert().success();

    // assert files have been created
    assert!(new_dir.exists());
    assert!(new_dir.join("README.md").exists());
    assert!(new_dir.join(".gitignore").exists());
    assert!(new_dir.join("config.toml").exists());

    // cleanup
    std::fs::remove_dir_all(new_dir).unwrap();
}


#[test]
fn create_new_project_fails_if_dir_exists() {
    let new_dir = get_tests_dir().join("create_new_project_fails_if_dir_exists");
    std::fs::create_dir(&new_dir).unwrap();

    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("new").arg(&new_dir).assert().failure().stderr(predicates::str::contains("Directory already exists"));

    // cleanup
    std::fs::remove_dir_all(new_dir).unwrap();
}

fn get_tests_dir() -> std::path::PathBuf {
    // get tmp dir
    let path = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));
    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path
}
