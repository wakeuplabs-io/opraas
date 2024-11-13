use std::process::Command;
use crate::{domain, system};

pub struct DockerReleaseRepository;

impl DockerReleaseRepository {
    pub fn new() -> Self {
        Self
    }

    fn exists(&self, artifact: &domain::Artifact) -> bool {
        !system::execute_command(
            Command::new("docker")
                .arg("images")
                .arg("-q")
                .arg(artifact.name()),
        )
        .unwrap()
        .is_empty()
    }
}

impl domain::TReleaseRepository for DockerReleaseRepository {
    fn pull(
        &self,
        release: &domain::Release,
    ) -> Result<(), Box<dyn std::error::Error>> {
        system::execute_command(
            Command::new("docker")
                .arg("pull")
                .arg(release.build_artifact_uri()),
        )?;

        Ok(())
    }

    fn create_for_artifact(
        &self,
        artifact: &domain::Artifact,
        release: &domain::Release,
    ) -> Result<(), Box<dyn std::error::Error>> {
        system::execute_command(
            Command::new("docker")
                .arg("tag")
                .arg(artifact.name())
                .arg(release.build_artifact_uri()),
        )?;

        system::execute_command(
            Command::new("docker")
                .arg("push")
                .arg(release.build_artifact_uri()),
        )?;

        Ok(())
    }
}
