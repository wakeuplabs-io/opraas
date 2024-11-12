use std::process::Command;

use crate::system;

use super::TArtifactBuilder;

pub struct DockerArtifactBuilder;

impl DockerArtifactBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl TArtifactBuilder for DockerArtifactBuilder {
    fn build_artifact(
        &self,
        artifact: &crate::domain::artifact::Artifact,
    ) -> Result<(), Box<dyn std::error::Error>> {
        system::execute_command(
            Command::new("docker")
                .arg("build")
                .arg("-t")
                .arg("demo")
                .arg("-f")
                .arg(artifact.dockerfile())
                .arg(".")
                .current_dir(artifact.context()),
        )?;

        Ok(())
    }
}
