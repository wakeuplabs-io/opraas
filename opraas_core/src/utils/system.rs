use std::process::Command;
use mockall::automock;

pub struct System;

impl System {
    pub fn new() -> Self {
        Self
    }   
}

#[automock]
pub trait TSystem: Send + Sync {
    fn execute_command(&self, command: &mut Command) -> Result<String, String>;
}


impl TSystem for System {
    fn execute_command(&self, command: &mut Command) -> Result<String, String> {
        let output = command.output().map_err(|e| format!("Failed to execute command: {}", e))?;
    
        if output.status.success() {
            // Convert the output to a String
            let result =   String::from_utf8_lossy(&output.stdout)
                .to_string();
            Ok(result)
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            Err(error_message.to_string())
        }
    }
}