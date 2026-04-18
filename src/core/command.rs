//! 命令执行工具
//! 提供统一的命令构建和执行功能

use std::process::Command;
use super::analyzer::AnalyzerError;

/// 命令构建器
/// 用于构建和执行外部命令
pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
    verbose: bool,
}

impl CommandBuilder {
    /// 创建新的命令构建器
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            verbose: true,
        }
    }

    /// 添加单个参数
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// 添加多个参数
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args.extend(args);
        self
    }

    /// 根据条件添加参数
    pub fn condition(mut self, condition: bool, arg: impl Into<String>) -> Self {
        if condition {
            self.args.push(arg.into());
        }
        self
    }

    /// 设置是否输出执行信息
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// 构建命令向量
    pub fn build(self) -> Vec<String> {
        let mut cmd = vec![self.program];
        cmd.extend(self.args);
        cmd
    }

    /// 执行命令并捕获输出
    pub fn execute(&self) -> Result<String, AnalyzerError> {
        if self.verbose {
            println!("Running: {} {}", self.program, self.args.join(" "));
        }

        let output = Command::new(&self.program)
            .args(&self.args)
            .output()
            .map_err(|e| {
                AnalyzerError::CommandFailed(format!(
                    "Failed to execute {}: {}",
                    self.program, e
                ))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("{}{}", stdout, stderr))
    }

    /// 执行命令但不捕获输出
    pub fn execute_silent(&self) -> Result<(), AnalyzerError> {
        if self.verbose {
            println!("Running: {} {}", self.program, self.args.join(" "));
        }

        Command::new(&self.program)
            .args(&self.args)
            .output()
            .map_err(|e| {
                AnalyzerError::CommandFailed(format!(
                    "Failed to execute {}: {}",
                    self.program, e
                ))
            })?;

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
}
