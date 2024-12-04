use std::process::Command;

use crate::{domain, system};

pub struct DockerArtifactRepository;

// implementations ==========================================

impl DockerArtifactRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::TArtifactRepository for DockerArtifactRepository {
    fn create(&self, artifact: &domain::Artifact) -> Result<(), Box<dyn std::error::Error>> {
        system::execute_command(
            Command::new("docker")
                .arg("build")
                .arg("-t")
                .arg(artifact.name())
                .arg("-f")
                .arg(artifact.dockerfile())
                .arg(".")
                .current_dir(artifact.context()),
            false,
        )?;

        Ok(())
    }

    fn exists(&self, artifact: &domain::Artifact) -> bool {
        !system::execute_command(
            Command::new("docker")
                .arg("images")
                .arg("-q")
                .arg(artifact.name()),
            true,
        )
        .unwrap()
        .is_empty()
    }
}
