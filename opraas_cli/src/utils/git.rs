use std::process::Command;

pub struct Git {
    system: Box<dyn crate::system::TSystem>,
}

impl Git {
    pub fn new() -> Self {
        Self {
            system: Box::new(crate::system::System::new()),
        }
    }
}

pub trait TGit {
    fn has_uncommitted_changes(&self, git_path: &str) -> bool;
    fn tag_release(
        &self,
        git_path: &str,
        release_tag: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl TGit for Git {
    fn has_uncommitted_changes(&self, git_path: &str) -> bool {
        let output = self
            .system
            .execute_command(
                Command::new("git")
                    .arg("status")
                    .arg("--porcelain")
                    .current_dir(git_path),
            )
            .unwrap();

        !output.is_empty()
    }

    fn tag_release(
        &self,
        git_path: &str,
        release_tag: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.system.execute_command(
            Command::new("git")
                .arg("tag")
                .arg(release_tag)
                .current_dir(git_path),
        )?;
        Ok(())
    }
}
