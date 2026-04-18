//! command execution tool
//! Provides unified command construction and execution functions and supports cross-platform command lookup

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::analyzer::AnalyzerError;

/// Default command timeout (5 minutes)
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(300);

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

    /// Set command execution timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add a single parameter
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add multiple parameters
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args.extend(args);
        self
    }

    /// Add parameters based on conditions
    pub fn condition(mut self, condition: bool, arg: impl Into<String>) -> Self {
        if condition {
            self.args.push(arg.into());
        }
        self
    }

    /// Set whether to output execution information
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Build command vectors
    pub fn build(self) -> Vec<String> {
        let mut cmd = vec![self.program];
        cmd.extend(self.args);
        cmd
    }

    /// Get the program name (for cross-platform resolution)
    fn resolve_program(&self) -> Result<String, AnalyzerError> {
        // Attempt to resolve the command path
        if let Some(resolved) = resolve_command(&self.program) {
            return Ok(resolved.to_string_lossy().to_string());
        }

        // If parsing fails, return the original program name (let the system try)
        Ok(self.program.clone())
    }

    /// Execute command and capture output (with timeout)
    pub fn execute(&self) -> Result<String, AnalyzerError> {
        let program = self.resolve_program()?;

        if self.verbose {
            println!("Running: {} {}", program, self.args.join(" "));
        }

        self.execute_with_timeout(&program, None)
    }

    /// Internal method: Execute commands with timeout control
    fn execute_with_timeout(
        &self,
        program: &str,
        dir: Option<&PathBuf>,
    ) -> Result<String, AnalyzerError> {
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

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout, stderr);

        if !output.status.success() {
            let location = dir.map(|d| format!(" in directory {}", d.display()))
                .unwrap_or_default();
            return Err(AnalyzerError::CommandFailed(format!(
                "Command '{}' failed with exit code {:?}{}\nOutput: {}",
                self.program,
                output.status.code(),
                location,
                combined.trim()
            )));
        }

        Ok(combined)
    }

    /// Executes the command in the specified directory and captures the output (with timeout)
    pub fn execute_in_dir(&self, dir: &PathBuf) -> Result<String, AnalyzerError> {
        let program = self.resolve_program()?;

        if self.verbose {
            println!(
                "Running in {}: {} {}",
                dir.display(),
                program,
                self.args.join(" ")
            );
        }

        let (tx, rx) = mpsc::channel();
        let program = program.to_string();
        let args = self.args.clone();
        let dir_for_thread = dir.clone();
        let timeout = self.timeout;

        thread::spawn(move || {
            let output = Command::new(&program)
                .args(&args)
                .current_dir(&dir_for_thread)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
            let _ = tx.send(output);
        });

        let result = rx.recv_timeout(timeout).map_err(|_| {
            AnalyzerError::Timeout(timeout)
        })?;

        let output = result.map_err(|e| {
            AnalyzerError::CommandFailed(format!(
                "Failed to execute {} in {}: {}. Hint: Make sure '{}' is installed and in PATH",
                self.program,
                dir.display(),
                e,
                self.program
            ))
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout, stderr);

        if !output.status.success() {
            return Err(AnalyzerError::CommandFailed(format!(
                "Command '{}' failed with exit code {:?} in directory {}\nOutput: {}",
                self.program,
                output.status.code(),
                dir.display(),
                combined.trim()
            )));
        }

        Ok(combined)
    }

    /// Execute command without capturing output (with timeout)
    pub fn execute_silent(&self) -> Result<(), AnalyzerError> {
        let program = self.resolve_program()?;

        if self.verbose {
            println!("Running: {} {}", program, self.args.join(" "));
        }

        let (tx, rx) = mpsc::channel();
        let program = program.to_string();
        let args = self.args.clone();
        let timeout = self.timeout;

        thread::spawn(move || {
            let output = Command::new(&program)
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
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

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AnalyzerError::CommandFailed(format!(
                "Command '{}' failed with exit code {:?}\nStderr: {}",
                self.program,
                output.status.code(),
                stderr.trim()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command() {
        let cmd = CommandBuilder::new("cargo")
            .arg("check")
            .arg("--all-targets")
            .condition(true, "--verbose")
            .condition(false, "--quiet")
            .build();

        assert_eq!(cmd.len(), 4);
        assert_eq!(cmd[0], "cargo");
        assert_eq!(cmd[1], "check");
        assert_eq!(cmd[2], "--all-targets");
        assert_eq!(cmd[3], "--verbose");
    }

    #[test]
    fn test_command_builder_empty() {
        let cmd = CommandBuilder::new("npm").build();
        assert_eq!(cmd.len(), 1);
        assert_eq!(cmd[0], "npm");
    }

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
