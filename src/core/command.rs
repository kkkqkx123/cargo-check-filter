//! 命令执行工具
//! 提供统一的命令构建和执行功能，支持跨平台命令查找

use std::path::PathBuf;
use std::process::Command;

use super::analyzer::AnalyzerError;

/// 获取命令的完整路径（跨平台）
/// 在 Windows 上，会优先查找 .cmd, .bat, .exe 等可执行扩展名
pub fn resolve_command(cmd: &str) -> Option<PathBuf> {
    // 如果已经是绝对路径或相对路径，直接返回
    if cmd.contains('/') || cmd.contains('\\') {
        return Some(PathBuf::from(cmd));
    }

    // 使用 which/where 命令查找
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
        // 在 Windows 上，优先选择有扩展名的可执行文件
        // 优先级: .cmd > .bat > .exe > 其他
        let priority = [".cmd", ".bat", ".exe"];
        for ext in &priority {
            if let Some(path) = paths.iter().find(|p| {
                p.extension()
                    .map(|e| e.to_string_lossy().to_lowercase() == ext.trim_start_matches('.'))
                    .unwrap_or(false)
            }) {
                return Some(path.clone());
            }
        }
    }

    // 默认返回第一个找到的路径
    paths.into_iter().next()
}

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

    /// 获取程序名称（用于跨平台解析）
    fn resolve_program(&self) -> Result<String, AnalyzerError> {
        // 尝试解析命令路径
        if let Some(resolved) = resolve_command(&self.program) {
            return Ok(resolved.to_string_lossy().to_string());
        }

        // 如果解析失败，返回原始程序名（让系统尝试）
        Ok(self.program.clone())
    }

    /// 执行命令并捕获输出
    pub fn execute(&self) -> Result<String, AnalyzerError> {
        let program = self.resolve_program()?;

        if self.verbose {
            println!("Running: {} {}", program, self.args.join(" "));
        }

        let output = Command::new(&program)
            .args(&self.args)
            .output()
            .map_err(|e| {
                AnalyzerError::CommandFailed(format!(
                    "Failed to execute {}: {}. Hint: Make sure '{}' is installed and in PATH",
                    self.program, e, self.program
                ))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("{}{}", stdout, stderr))
    }

    /// 在指定目录执行命令并捕获输出
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

        let output = Command::new(&program)
            .args(&self.args)
            .current_dir(dir)
            .output()
            .map_err(|e| {
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

        Ok(format!("{}{}", stdout, stderr))
    }

    /// 执行命令但不捕获输出
    pub fn execute_silent(&self) -> Result<(), AnalyzerError> {
        let program = self.resolve_program()?;

        if self.verbose {
            println!("Running: {} {}", program, self.args.join(" "));
        }

        Command::new(&program)
            .args(&self.args)
            .output()
            .map_err(|e| {
                AnalyzerError::CommandFailed(format!(
                    "Failed to execute {}: {}. Hint: Make sure '{}' is installed and in PATH",
                    self.program, e, self.program
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

    #[test]
    fn test_resolve_command_cargo() {
        // cargo 应该能找到
        let resolved = resolve_command("cargo");
        assert!(
            resolved.is_some(),
            "cargo should be found in PATH"
        );
    }

    #[test]
    fn test_resolve_command_nonexistent() {
        // 不存在的命令应该返回 None
        let resolved = resolve_command("this_command_definitely_does_not_exist_12345");
        assert!(resolved.is_none());
    }
}
