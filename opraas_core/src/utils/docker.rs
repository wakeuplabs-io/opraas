use std::process::Command;

use mockall::automock;

use super::system;

pub struct DockerBuilder {
    system: Box<dyn system::TSystem>,
}

#[automock]
pub trait TDockerBuilder: Send + Sync {
    fn build(
        &self,
        cwd: &str,
        dockerfile: &str,
        tag: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn push(&self, src_tag: &str, dest_tag: &str, repository: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl DockerBuilder {
    pub fn new() -> Self {
        DockerBuilder {
            system: Box::new(system::System::new()),
        }
    }
}

impl TDockerBuilder for DockerBuilder {
    fn build(
        &self,
        context: &str,
        dockerfile: &str,
        tag: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::new("docker");
        command
            .current_dir(context)
            .arg("build")
            .arg("-f")
            .arg(dockerfile)
            .arg("-t")
            .arg(tag)
            .arg(context);
        self.system.execute_command(&mut command)?;

        Ok(())
    }

    fn push(&self, src_tag: &str, dest_tag: &str, repository: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::new("docker");
        command
            .arg("tag")
            .arg(src_tag)
            .arg(format!("{}/{}", repository, dest_tag));
        self.system.execute_command(&mut command)?;

        let mut command = Command::new("docker");
        command.arg("push").arg(format!("{}/{}", repository, dest_tag));
        self.system.execute_command(&mut command)?;

        Ok(())
    }
}
