//! 配置系统
//! 支持多层级配置：内置默认 < 二进制目录 < 项目
//! 注意：不使用任何用户目录配置，避免配置分散

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::ReportFormat;

/// 配置根结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// 配置版本
    #[serde(default = "default_version")]
    pub version: String,

    /// 全局设置
    #[serde(default)]
    pub global: GlobalConfig,

    /// 命令定义
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,

    /// 技术栈特定配置
    #[serde(rename = "tech_stack", default)]
    pub tech_stacks: HashMap<String, TechStackConfig>,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// 默认输出格式
    #[serde(default = "default_format")]
    pub default_format: String,

    /// 是否默认过滤警告
    #[serde(default)]
    pub filter_warnings: bool,

    /// 默认报告输出路径
    pub default_output: Option<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            default_format: default_format(),
            filter_warnings: false,
            default_output: None,
        }
    }
}

fn default_format() -> String {
    "markdown".to_string()
}

/// 命令配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// 实际执行的命令
    pub exec: String,

    /// 命令描述
    pub description: Option<String>,

    /// 适用的技术栈
    #[serde(default)]
    pub tech_stacks: Vec<String>,

    /// 是否启用
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

/// 技术栈特定配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TechStackConfig {
    /// 命令覆盖
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,

    /// 脚本名称映射
    #[serde(default)]
    pub scripts: HashMap<String, String>,

    /// 测试框架类型
    pub test_framework: Option<String>,
}

impl Config {
    /// 创建空配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 加载配置（按优先级合并所有层级）
    /// 优先级：内置默认 < 二进制目录 < 项目
    /// 注意：不使用用户目录配置
    pub fn load(project_path: &Path) -> Result<Self, ConfigError> {
        let mut config = Self::default();

        // 1. 加载内置默认值
        config.merge(Self::embedded_defaults());

        // 2. 加载二进制目录配置
        if let Some(binary_dir_config) = Self::load_from_binary_dir()? {
            config.merge(binary_dir_config);
        }

        // 3. 加载项目级配置（最高优先级）
        if let Some(project_config) = Self::load_from_project(project_path)? {
            config.merge(project_config);
        }

        Ok(config)
    }

    /// 仅从项目路径加载配置（用于测试或简化场景）
    pub fn load_project_only(project_path: &Path) -> Result<Self, ConfigError> {
        let mut config = Self::default();
        config.merge(Self::embedded_defaults());

        if let Some(project_config) = Self::load_from_project(project_path)? {
            config.merge(project_config);
        }

        Ok(config)
    }

    /// 内置默认值
    fn embedded_defaults() -> Self {
        let mut commands = HashMap::new();

        // Cargo 命令
        commands.insert(
            "check".to_string(),
            CommandConfig {
                exec: "cargo check".to_string(),
                description: Some("Fast syntax and type checking".to_string()),
                tech_stacks: vec!["cargo".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "clippy".to_string(),
            CommandConfig {
                exec: "cargo clippy".to_string(),
                description: Some("Run Clippy linter".to_string()),
                tech_stacks: vec!["cargo".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "clippy-all".to_string(),
            CommandConfig {
                exec: "cargo clippy --all-targets --all-features".to_string(),
                description: Some("Run Clippy on all targets and features".to_string()),
                tech_stacks: vec!["cargo".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "check-test".to_string(),
            CommandConfig {
                exec: "cargo check --tests".to_string(),
                description: Some("Check test code syntax and types".to_string()),
                tech_stacks: vec!["cargo".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "test".to_string(),
            CommandConfig {
                exec: "cargo test".to_string(),
                description: Some("Run tests".to_string()),
                tech_stacks: vec!["cargo".to_string()],
                enabled: true,
            },
        );

        // NPM 命令
        commands.insert(
            "lint".to_string(),
            CommandConfig {
                exec: "npm run lint".to_string(),
                description: Some("Run linter".to_string()),
                tech_stacks: vec!["npm".to_string(), "pnpm".to_string(), "yarn".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "type-check".to_string(),
            CommandConfig {
                exec: "npm run type-check".to_string(),
                description: Some("Run TypeScript type checker".to_string()),
                tech_stacks: vec!["npm".to_string(), "pnpm".to_string(), "yarn".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "audit".to_string(),
            CommandConfig {
                exec: "npm audit".to_string(),
                description: Some("Audit dependencies for vulnerabilities".to_string()),
                tech_stacks: vec!["npm".to_string(), "pnpm".to_string(), "yarn".to_string()],
                enabled: true,
            },
        );

        // Mypy 命令
        commands.insert(
            "mypy".to_string(),
            CommandConfig {
                exec: "mypy".to_string(),
                description: Some("Run mypy type checker".to_string()),
                tech_stacks: vec!["mypy".to_string()],
                enabled: true,
            },
        );

        commands.insert(
            "mypy-strict".to_string(),
            CommandConfig {
                exec: "mypy --strict".to_string(),
                description: Some("Run mypy in strict mode".to_string()),
                tech_stacks: vec!["mypy".to_string()],
                enabled: true,
            },
        );

        Self {
            version: "1.0".to_string(),
            global: GlobalConfig::default(),
            commands,
            tech_stacks: HashMap::new(),
        }
    }

    /// 从二进制文件所在目录加载配置
    fn load_from_binary_dir() -> Result<Option<Self>, ConfigError> {
        let exe_path = std::env::current_exe()
            .map_err(|e| ConfigError::IoError(format!("Failed to get executable path: {}", e)))?;

        let binary_dir = exe_path
            .parent()
            .ok_or_else(|| ConfigError::IoError("Failed to get binary directory".to_string()))?;

        let config_path = binary_dir.join("analyzer.toml");
        Self::load_from_file(&config_path)
    }

    /// 从项目路径加载配置
    fn load_from_project(project_path: &Path) -> Result<Option<Self>, ConfigError> {
        // 优先查找隐藏文件 .analyzer.toml
        let hidden_config = project_path.join(".analyzer.toml");
        if hidden_config.exists() {
            return Self::load_from_file(&hidden_config);
        }

        // 然后查找 analyzer.toml
        let config_path = project_path.join("analyzer.toml");
        Self::load_from_file(&config_path)
    }

    /// 从文件加载配置
    fn load_from_file(path: &Path) -> Result<Option<Self>, ConfigError> {
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(format!("Failed to read {:?}: {}", path, e)))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse {:?}: {}", path, e)))?;

        Ok(Some(config))
    }

    /// 合并另一个配置（高优先级覆盖低优先级）
    fn merge(&mut self, other: Self) {
        // 合并全局配置
        if other.global.default_format != default_format() {
            self.global.default_format = other.global.default_format;
        }
        self.global.filter_warnings = other.global.filter_warnings;
        if other.global.default_output.is_some() {
            self.global.default_output = other.global.default_output;
        }

        // 合并命令（高优先级覆盖）
        for (name, cmd_config) in other.commands {
            self.commands.insert(name, cmd_config);
        }

        // 合并技术栈配置
        for (stack_name, stack_config) in other.tech_stacks {
            self.tech_stacks
                .entry(stack_name)
                .and_modify(|existing| {
                    // 合并命令
                    for (cmd_name, cmd_config) in &stack_config.commands {
                        existing.commands.insert(cmd_name.clone(), cmd_config.clone());
                    }
                    // 合并脚本
                    for (script_name, script_value) in &stack_config.scripts {
                        existing.scripts.insert(script_name.clone(), script_value.clone());
                    }
                    // 覆盖测试框架
                    if stack_config.test_framework.is_some() {
                        existing.test_framework = stack_config.test_framework.clone();
                    }
                })
                .or_insert(stack_config);
        }
    }

    /// 获取命令配置（考虑技术栈特定覆盖）
    pub fn get_command(&self, tech_stack: &str, command_name: &str) -> Option<&CommandConfig> {
        // 1. 先查技术栈特定配置
        if let Some(stack_config) = self.tech_stacks.get(tech_stack) {
            if let Some(cmd) = stack_config.commands.get(command_name) {
                if cmd.enabled {
                    return Some(cmd);
                }
            }
        }

        // 2. 再查全局命令
        if let Some(cmd) = self.commands.get(command_name) {
            if cmd.enabled && cmd.tech_stacks.iter().any(|s| s == tech_stack) {
                return Some(cmd);
            }
        }

        None
    }

    /// 获取所有可用的命令名称
    pub fn get_available_commands(&self, tech_stack: &str) -> Vec<&String> {
        self.commands
            .iter()
            .filter(|(_, config)| {
                config.enabled && config.tech_stacks.iter().any(|s| s == tech_stack)
            })
            .map(|(name, _)| name)
            .collect()
    }

    /// 获取技术栈的脚本映射
    pub fn get_script_mapping(&self, tech_stack: &str, script_name: &str) -> Option<&String> {
        self.tech_stacks
            .get(tech_stack)
            .and_then(|stack| stack.scripts.get(script_name))
    }

    /// 获取技术栈的测试框架
    pub fn get_test_framework(&self, tech_stack: &str) -> Option<&String> {
        self.tech_stacks
            .get(tech_stack)
            .and_then(|stack| stack.test_framework.as_ref())
    }
}

/// 配置错误
#[derive(Debug)]
pub enum ConfigError {
    IoError(String),
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(msg) => write!(f, "Config IO error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Config parse error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_defaults() {
        let config = Config::embedded_defaults();
        assert!(!config.commands.is_empty());
        assert!(config.commands.contains_key("check"));
        assert!(config.commands.contains_key("test"));
    }

    #[test]
    fn test_get_command() {
        let config = Config::embedded_defaults();

        let check_cmd = config.get_command("cargo", "check");
        assert!(check_cmd.is_some());
        assert_eq!(check_cmd.unwrap().exec, "cargo check");

        let lint_cmd = config.get_command("npm", "lint");
        assert!(lint_cmd.is_some());
        assert_eq!(lint_cmd.unwrap().exec, "npm run lint");
    }

    #[test]
    fn test_merge() {
        let mut base = Config::embedded_defaults();

        let mut override_config = Config::new();
        override_config.commands.insert(
            "check".to_string(),
            CommandConfig {
                exec: "cargo check --all-targets".to_string(),
                description: Some("Custom check".to_string()),
                tech_stacks: vec!["cargo".to_string()],
                enabled: true,
            },
        );

        base.merge(override_config);

        let check_cmd = base.get_command("cargo", "check");
        assert_eq!(check_cmd.unwrap().exec, "cargo check --all-targets");
    }
}
