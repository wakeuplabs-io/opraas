use std::process::Command;

use crate::infra::system::{System, TSystem};

use super::TVersionControl;

pub struct Git {
    system: Box<dyn TSystem>,
}

// implementations ================================================

impl Git {
    pub fn new() -> Self {
        Self {
            system: Box::new(System::new()),
        }
    }
}

impl TVersionControl for Git {
    fn tag_release(&self, git_path: &str, release_tag: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.system.execute_command(
            Command::new("git")
                .arg("tag")
                .arg(release_tag)
                .current_dir(git_path),
        )?;
        Ok(())
    }
}
