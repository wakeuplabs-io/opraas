use log::debug;
use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn execute_command(command: &mut Command, silent: bool) -> Result<String, String> {
    debug!("Executing command: {:?}", command);

    let status: std::process::ExitStatus;
    let result: String;

    if silent {
        let output = command
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        result = String::from_utf8_lossy(&output.stdout).to_string();
        status = output.status;
    } else {
        // Configure the command to capture output streams
        let mut child = command
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn command");

        let stdout = BufReader::new(child.stdout.take().unwrap());

        // Spawn a thread to handle stdout streaming
        let stdout_thread = std::thread::spawn(move || {
            let mut local_output = String::new();
            for line in stdout.lines() {
                if let Ok(line) = line {
                    debug!("{}", line);
                    local_output.push_str(&line);
                    local_output.push('\n');
                }
            }
            local_output
        });

        result = (&stdout_thread.join().unwrap()).to_string();

        status = child
            .wait()
            .map_err(|e| format!("Failed to wait for child process: {}", e))?;
    }

    if status.success() {
        return Ok(result);
    } else {
        return Err(format!("Command exited with non-zero status: {}", status));
    }
}

pub fn copy_and_overwrite(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    if dest.exists() {
        fs::remove_file(dest)?;
    }

    if src.is_file() {
        fs::copy(src, dest)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a file.",
        ));
    }

    Ok(())
}
