use super::TReleaseRunner;
use crate::{domain::Release, system::execute_command};
use std::{collections::HashMap, path::Path, process::Command};

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
        volume: &Path,
        env: HashMap<&str, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let env_args: Vec<String> = env
            .iter()
            .map(|(key, value)| format!("-e {}={}", key, value))
            .collect();

        execute_command(Command::new("docker").arg("pull").arg(release.uri()))?;

        execute_command(
            Command::new("docker")
                .arg("run")
                .arg("--rm")
                .arg("-v")
                .args(env_args)
                .arg(format!("{}:{}", volume.display(), "/shared"))
                .arg(release.uri()),
        )?;

        Ok(())
    }
}
