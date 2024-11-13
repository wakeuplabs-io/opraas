use super::TReleaseRunner;
use crate::{domain::Release, system::execute_command};
use std::process::Command;

pub struct DockerArtifactRunner;

// implementations =============================================

impl DockerArtifactRunner {
    pub fn new() -> Self {
        Self
    }
}

impl TReleaseRunner for DockerArtifactRunner {
    fn run(
        &self,
        release: &Release,
        volume: &str,
        args: Vec<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(
            Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .arg(format!("{}:{}", volume, "/data"))
                .arg(release.build_artifact_uri())
                .args(args),
        )?;

        Ok(())
    }
}
