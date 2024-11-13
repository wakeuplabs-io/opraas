use std::process::Command;
use crate::{domain::Artifact, system::execute_command};
use super::TArtifactRunner;

pub struct  DockerArtifactRunner ;

impl  DockerArtifactRunner {
    pub fn new() -> Self {
        Self
    }
}

impl TArtifactRunner for DockerArtifactRunner {
    fn run_artifact(&self, artifact: &Artifact, volume: &str, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(
            Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .arg(format!("{}:{}", volume, "/data"))
                .arg(artifact.name())
                .args(args)
        )?;

        Ok(())
    }
}