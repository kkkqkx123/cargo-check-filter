//! 分析器 trait 定义
//! 定义了构建工具分析器的接口

use std::path::Path;
use super::types::{AnalysisResult, AnalyzeOptions};
use super::parser::OutputParser;
use super::config::Config;

/// 分析器错误类型
#[derive(Debug)]
pub enum AnalyzerError {
    CommandFailed(String),
    ParseError(String),
    IoError(std::io::Error),
    NotApplicable,
}

impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzerError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            AnalyzerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AnalyzerError::IoError(e) => write!(f, "IO error: {}", e),
            AnalyzerError::NotApplicable => write!(f, "Analyzer not applicable for this project"),
        }
    }
}

impl std::error::Error for AnalyzerError {}

impl From<std::io::Error> for AnalyzerError {
    fn from(e: std::io::Error) -> Self {
        AnalyzerError::IoError(e)
    }
}

/// 构建工具分析器 trait
/// 实现此 trait 以支持新的构建工具
pub trait BuildAnalyzer: Send + Sync {
    /// 获取技术栈名称
    fn name(&self) -> &str;

    /// 获取支持的命令别名
    fn supported_commands(&self) -> Vec<&str>;

    /// 检查当前目录是否适用此分析器
    fn is_applicable(&self, project_path: &Path) -> bool;

    /// 运行分析命令
    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError>;

    /// 获取解析器
    fn parser(&self) -> &dyn OutputParser;

    /// 设置配置（用于配置驱动模式）
    fn set_config(&mut self, config: Config) {
        // 默认空实现，向后兼容
        let _ = config;
    }

    /// 获取配置（如果已设置）
    fn config(&self) -> Option<&Config> {
        None
    }
}

/// 插件注册表
pub struct PluginRegistry {
    analyzers: Vec<Box<dyn BuildAnalyzer>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    /// 注册分析器
    pub fn register(&mut self, analyzer: Box<dyn BuildAnalyzer>) {
        self.analyzers.push(analyzer);
    }

    /// 根据命令名称获取分析器
    pub fn get(&self, command: &str) -> Option<&dyn BuildAnalyzer> {
        self.analyzers
            .iter()
            .find(|a| {
                a.name() == command
                    || a.supported_commands().contains(&command)
            })
            .map(|b| b.as_ref())
    }

    /// 自动检测项目适用的分析器
    pub fn detect(&self, path: &Path) -> Vec<&dyn BuildAnalyzer> {
        self.analyzers
            .iter()
            .filter(|a| a.is_applicable(path))
            .map(|b| b.as_ref())
            .collect()
    }

    /// 列出所有已注册的分析器
    pub fn list(&self) -> Vec<&str> {
        self.analyzers.iter().map(|a| a.name()).collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
