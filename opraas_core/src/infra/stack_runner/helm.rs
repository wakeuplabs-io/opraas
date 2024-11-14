use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};
use crate::system;
use super::stack_runner::TStackRunner;

pub struct HelmStackRunner {
    release_name: String,
    helm_root: String,
    namespace: String,
}

// implementations ============================================================

impl HelmStackRunner {
    pub fn new(helm_root: &str, release_name: &str, namespace: &str) -> Self {
        Self {
            release_name: release_name.to_string(),
            helm_root: helm_root.to_string(),
            namespace: namespace.to_string(),
        }
    }
}

impl TStackRunner for HelmStackRunner {
    fn run(
        &self,
        values_file: &str,
        overrides: HashMap<&str, &str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::new("helm");
        command.arg("install").arg(&self.helm_root);
        command.arg("-f").arg(values_file);
        command.arg("--namespace").arg(&self.namespace);

        for (key, value) in overrides.iter() {
            command.arg(format!("--set {}={}", key, value));
        }

        command.exec();

        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::execute_command(
            Command::new("helm")
                .arg("uninstall")
                .arg(&self.release_name)
                .arg("--namespace")
                .arg(&self.namespace),
        )?;

        Ok(())
    }
}
