//! Configuration system
//! Multi-level configuration support: built-in default < binary directory < project
//! Note: Do not use any user directory configuration to avoid configuration fragmentation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuring the root structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Configuration version
    #[serde(default = "default_version")]
    pub version: String,

    /// global setting
    #[serde(default)]
    pub global: GlobalConfig,

    /// Command Definition
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,

    /// Technology Stack Specific Configuration
    #[serde(rename = "tech_stack", default)]
    pub tech_stacks: HashMap<String, TechStackConfig>,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// global configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Default Output Format
    #[serde(default = "default_format")]
    pub default_format: String,

    /// Whether to filter warnings by default
    #[serde(default)]
    pub filter_warnings: bool,

    /// Default report output path
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

/// instruction layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// Commands actually executed
    pub exec: String,

    /// Command Description
    pub description: Option<String>,

    /// applicable technology stack
    #[serde(default)]
    pub tech_stacks: Vec<String>,

    /// Enable or disable
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

/// Technology Stack Specific Configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TechStackConfig {
    /// Command Override
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,

    /// Script Name Mapping
    #[serde(default)]
    pub scripts: HashMap<String, String>,

    /// Types of test frameworks
    pub test_framework: Option<String>,
}

impl Config {
    /// Load configuration (merge all tiers by priority)
    /// Priority: Built-in Defaults < Binary Directories < Projects
    /// Note: Do not use the user directory configuration
    pub fn load(project_path: &Path) -> Result<Self, ConfigError> {
        let mut config = Self::default();

        // 1. Loading built-in defaults
        config.merge(Self::embedded_defaults());

        // 2. Load binary directory configuration
        if let Some(binary_dir_config) = Self::load_from_binary_dir()? {
            config.merge(binary_dir_config);
        }

        // 3. Load project level configuration (highest priority)
        if let Some(project_config) = Self::load_from_project(project_path)? {
            config.merge(project_config);
        }

        Ok(config)
    }

    /// Get the command execution string for the given tech stack and command name
    /// Priority: tech_stack specific command > global command
    pub fn get_command_exec(&self, tech_stack: &str, command_name: &str) -> Option<String> {
        // First check tech_stack specific command
        if let Some(stack_config) = self.tech_stacks.get(tech_stack) {
            if let Some(cmd_config) = stack_config.commands.get(command_name) {
                if cmd_config.enabled {
                    return Some(cmd_config.exec.clone());
                }
            }
        }

        // Then check global command
        if let Some(cmd_config) = self.commands.get(command_name) {
            if cmd_config.enabled {
                return Some(cmd_config.exec.clone());
            }
        }

        None
    }

    /// Check if a command is enabled for the given tech stack
    pub fn is_command_enabled(&self, tech_stack: &str, command_name: &str) -> bool {
        // First check tech_stack specific command
        if let Some(stack_config) = self.tech_stacks.get(tech_stack) {
            if let Some(cmd_config) = stack_config.commands.get(command_name) {
                return cmd_config.enabled;
            }
        }

        // Then check global command
        if let Some(cmd_config) = self.commands.get(command_name) {
            return cmd_config.enabled;
        }

        false
    }

    /// Get all available command names for a tech stack
    pub fn get_available_commands(&self, tech_stack: &str) -> Vec<String> {
        let mut commands = std::collections::HashSet::new();

        // Add global commands that apply to this tech stack
        for (name, cmd_config) in &self.commands {
            if cmd_config.enabled {
                if cmd_config.tech_stacks.contains(&tech_stack.to_string()) {
                    commands.insert(name.clone());
                }
            }
        }

        // Add tech_stack specific commands
        if let Some(stack_config) = self.tech_stacks.get(tech_stack) {
            for (name, cmd_config) in &stack_config.commands {
                if cmd_config.enabled {
                    commands.insert(name.clone());
                }
            }
        }

        commands.into_iter().collect()
    }

    /// Built-in defaults
    fn embedded_defaults() -> Self {
        let mut commands = HashMap::new();

        // Cargo orders
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

        // NPM Commands
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

        // Mypy command
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

    /// Load the configuration from the directory where the binary is located
    fn load_from_binary_dir() -> Result<Option<Self>, ConfigError> {
        let exe_path = std::env::current_exe()
            .map_err(|e| ConfigError::IoError(format!("Failed to get executable path: {}", e)))?;

        let binary_dir = exe_path
            .parent()
            .ok_or_else(|| ConfigError::IoError("Failed to get binary directory".to_string()))?;

        let config_path = binary_dir.join("analyzer.toml");
        Self::load_from_file(&config_path)
    }

    /// Load configuration from project path
    fn load_from_project(project_path: &Path) -> Result<Option<Self>, ConfigError> {
        // Prioritize hidden files .analyzer.toml
        let hidden_config = project_path.join(".analyzer.toml");
        if hidden_config.exists() {
            return Self::load_from_file(&hidden_config);
        }

        // Then look for analyzer.toml
        let config_path = project_path.join("analyzer.toml");
        Self::load_from_file(&config_path)
    }

    /// Load configuration from file
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

    /// Merge another configuration (high priority overrides low priority)
    pub fn merge(&mut self, other: Self) {
        // Merge Global Configuration
        if other.global.default_format != default_format() {
            self.global.default_format = other.global.default_format;
        }
        self.global.filter_warnings = other.global.filter_warnings;
        if other.global.default_output.is_some() {
            self.global.default_output = other.global.default_output;
        }

        // Merge command (high-priority override)
        for (name, cmd_config) in other.commands {
            self.commands.insert(name, cmd_config);
        }

        // Consolidate technology stack configurations
        for (stack_name, stack_config) in other.tech_stacks {
            self.tech_stacks
                .entry(stack_name)
                .and_modify(|existing| {
                    // merge command
                    for (cmd_name, cmd_config) in &stack_config.commands {
                        existing.commands.insert(cmd_name.clone(), cmd_config.clone());
                    }
                    // Merge Script
                    for (script_name, script_value) in &stack_config.scripts {
                        existing.scripts.insert(script_name.clone(), script_value.clone());
                    }
                    // Coverage Testing Framework
                    if stack_config.test_framework.is_some() {
                        existing.test_framework = stack_config.test_framework.clone();
                    }
                })
                .or_insert(stack_config);
        }
    }

}

/// misconfiguration
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
    fn test_merge() {
        let mut base = Config::embedded_defaults();

        let mut override_config = Config::default();
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

        let check_cmd = base.commands.get("check");
        assert_eq!(check_cmd.unwrap().exec, "cargo check --all-targets");
    }

    #[test]
    fn test_get_command_exec() {
        let config = Config::embedded_defaults();

        // Test getting global command
        let cmd = config.get_command_exec("cargo", "check");
        assert_eq!(cmd, Some("cargo check".to_string()));

        // Test getting command for different tech stack
        let cmd = config.get_command_exec("npm", "lint");
        assert_eq!(cmd, Some("npm run lint".to_string()));

        // Test non-existent command
        let cmd = config.get_command_exec("cargo", "nonexistent");
        assert_eq!(cmd, None);
    }

    #[test]
    fn test_is_command_enabled() {
        let config = Config::embedded_defaults();

        assert!(config.is_command_enabled("cargo", "check"));
        assert!(config.is_command_enabled("npm", "lint"));
    }

    #[test]
    fn test_get_available_commands() {
        let config = Config::embedded_defaults();

        let cargo_commands = config.get_available_commands("cargo");
        assert!(cargo_commands.contains(&"check".to_string()));
        assert!(cargo_commands.contains(&"clippy".to_string()));
        assert!(cargo_commands.contains(&"test".to_string()));

        let npm_commands = config.get_available_commands("npm");
        assert!(npm_commands.contains(&"lint".to_string()));
        assert!(npm_commands.contains(&"type-check".to_string()));
    }

    #[test]
    fn test_tech_stack_override() {
        let mut config = Config::embedded_defaults();

        // Add tech_stack specific override
        let mut stack_config = TechStackConfig::default();
        stack_config.commands.insert(
            "check".to_string(),
            CommandConfig {
                exec: "cargo check --all-targets --all-features".to_string(),
                description: Some("Custom check".to_string()),
                tech_stacks: vec![],
                enabled: true,
            },
        );
        config.tech_stacks.insert("cargo".to_string(), stack_config);

        // Tech_stack specific command should take precedence
        let cmd = config.get_command_exec("cargo", "check");
        assert_eq!(cmd, Some("cargo check --all-targets --all-features".to_string()));
    }
}
