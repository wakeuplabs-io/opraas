use std::process::Command;

use crate::{domain, system::execute_command};

pub struct DockerReleaseRepository;

// implementations ==========================================

impl DockerReleaseRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::TReleaseRepository for DockerReleaseRepository {
    fn exists(&self, artifact: &domain::Artifact) -> bool {
        !execute_command(
            Command::new("docker")
                .arg("images")
                .arg("-q")
                .arg(artifact.name()),
        )
        .unwrap()
        .is_empty()
    }

    fn pull(
        &self,
        artifact: &domain::Artifact,
        release: &domain::Release,
    ) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(
            Command::new("docker")
                .arg("pull")
                .arg(release.build_artifact_uri(artifact.name().to_string())),
        )?;

        Ok(())
    }

    fn create(
        &self,
        artifact: &domain::Artifact,
        release: &domain::Release,
    ) -> Result<(), Box<dyn std::error::Error>> {
        execute_command(
            Command::new("docker")
                .arg("tag")
                .arg(artifact.name())
                .arg(release.build_artifact_uri(artifact.name().to_string())),
        )?;

        execute_command(
            Command::new("docker")
                .arg("push")
                .arg(release.build_artifact_uri(artifact.name().to_string())),
        )?;

        Ok(())
    }
}
