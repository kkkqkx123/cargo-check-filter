//! command execution tool
//! Provides unified command construction and execution functions and supports cross-platform command lookup

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ExitStatus};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::analyzer::AnalyzerError;

/// Default command timeout (5 minutes)
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(300);

/// Command execution result
/// Contains the output and success status of the command
#[derive(Debug)]
pub struct CommandOutput {
    /// Standard output (available for testing and external use)
    #[allow(dead_code)]
    pub stdout: String,
    /// Standard error (available for testing and external use)
    #[allow(dead_code)]
    pub stderr: String,
    /// Combined stdout and stderr
    pub combined: String,
    pub status: ExitStatus,
}

impl CommandOutput {
    /// Check if the command was successful
    pub fn success(&self) -> bool {
        self.status.success()
    }

    /// Get the exit code
    pub fn code(&self) -> Option<i32> {
        self.status.code()
    }

    /// Get stdout output (available for testing and external use)
    #[allow(dead_code)]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Get stderr output (available for testing and external use)
    #[allow(dead_code)]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Get combined stdout and stderr output
    #[allow(dead_code)]
    pub fn combined(&self) -> &str {
        &self.combined
    }
}

/// Get full path to commands (cross-platform)
/// On Windows, executable extensions such as.cmd, .bat, and.exe are first found
pub fn resolve_command(cmd: &str) -> Option<PathBuf> {
    // If it is already an absolute path or contains a path separator, return directly
    let path = Path::new(cmd);
    if path.is_absolute() || path.components().count() > 1 {
        return Some(path.to_path_buf());
    }

    // Use the which/where command to find
    #[cfg(windows)]
    let check_cmd = "where";
    #[cfg(not(windows))]
    let check_cmd = "which";

    let output = Command::new(check_cmd).arg(cmd).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let paths: Vec<PathBuf> = stdout.lines().map(PathBuf::from).collect();

    #[cfg(windows)]
    {
        // On Windows, preference is given to executables with extensions
        // Priority: .cmd > .bat > .exe > Other
        let priority = ["cmd", "bat", "exe"];
        for ext in &priority {
            if let Some(path) = paths.iter().find(|p| {
                p.extension()
                    .map(|e| e.to_string_lossy().to_lowercase() == *ext)
                    .unwrap_or(false)
            }) {
                return Some(path.clone());
            }
        }
    }

    // The default returns the first found path
    paths.into_iter().next()
}

/// command builder
/// Used to build and execute external commands
pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
    verbose: bool,
    timeout: Duration,
}

impl CommandBuilder {
    /// Create a new command builder
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            verbose: true,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Create a command builder from an execution string (e.g., "cargo check --all-targets")
    pub fn from_exec_string(exec_str: &str) -> Self {
        let parts: Vec<&str> = exec_str.split_whitespace().collect();
        if parts.is_empty() {
            return Self::new("");
        }

        let mut builder = Self::new(parts[0]);
        for arg in &parts[1..] {
            builder = builder.arg(*arg);
        }
        builder
    }

    /// Add a single parameter
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Execute command and capture output (with timeout)
    /// Returns the combined stdout and stderr output
    pub fn execute(&self) -> Result<String, AnalyzerError> {
        let output = self.execute_with_status()?;
        Ok(output.combined)
    }

    /// Execute command and capture output with full status information
    /// This allows callers to check if the command succeeded and handle failures appropriately
    pub fn execute_with_status(&self) -> Result<CommandOutput, AnalyzerError> {
        let program = resolve_command(&self.program)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| self.program.clone());

        if self.verbose {
            println!("Running: {} {}", program, self.args.join(" "));
        }

        let (tx, rx) = mpsc::channel();
        let program = program.to_string();
        let args = self.args.clone();
        let timeout = self.timeout;

        thread::spawn(move || {
            let mut cmd = Command::new(&program);
            cmd.args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            let output = cmd.output();
            let _ = tx.send(output);
        });

        let result = rx.recv_timeout(timeout).map_err(|_| {
            AnalyzerError::Timeout(timeout)
        })?;

        let output = result.map_err(|e| {
            AnalyzerError::CommandFailed(format!(
                "Failed to execute {}: {}. Hint: Make sure '{}' is installed and in PATH",
                self.program, e, self.program
            ))
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let combined = format!("{}{}", stdout, stderr);

        Ok(CommandOutput {
            stdout,
            stderr,
            combined,
            status: output.status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_command_cargo() {
        // cargo should be able to find
        let resolved = resolve_command("cargo");
        assert!(
            resolved.is_some(),
            "cargo should be found in PATH"
        );
    }

    #[test]
    fn test_resolve_command_nonexistent() {
        // Command that does not exist should return None
        let resolved = resolve_command("this_command_definitely_does_not_exist_12345");
        assert!(resolved.is_none());
    }
}
