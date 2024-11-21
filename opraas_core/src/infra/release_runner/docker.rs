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
        let env_args: Vec<Vec<String>> = env
            .iter()
            .map(|(key, value)| vec!["--env".to_string(), format!("{}={}", key, value)])
            .collect();

        execute_command(
            Command::new("docker")
                .arg("run")
                .arg("--rm")
                .args(env_args.concat())
                .arg("-v")
                .arg(format!("{}:{}", volume.display(), "/shared"))
                .arg(release.uri()),
        )?;

        Ok(())
    }
}
