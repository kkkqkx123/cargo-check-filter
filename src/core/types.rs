//! 核心类型定义
//! 提供所有技术栈通用的类型

use std::collections::{HashMap, HashSet};

/// 问题级别
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IssueLevel {
    Error,
    Warning,
    Info,
    Hint,
}

impl std::fmt::Display for IssueLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueLevel::Error => write!(f, "error"),
            IssueLevel::Warning => write!(f, "warning"),
            IssueLevel::Info => write!(f, "info"),
            IssueLevel::Hint => write!(f, "hint"),
        }
    }
}

/// 问题位置
#[derive(Debug, Clone)]
pub struct Location {
    pub file_path: String,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
}

impl Location {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            line_number: None,
            column_number: None,
        }
    }

    pub fn with_line(mut self, line: u32) -> Self {
        self.line_number = Some(line);
        self
    }

    pub fn with_column(mut self, column: u32) -> Self {
        self.column_number = Some(column);
        self
    }
}

/// 问题信息
#[derive(Debug, Clone)]
pub struct Issue {
    pub level: IssueLevel,
    pub code: Option<String>,
    pub message: String,
    pub location: Location,
    pub context: Option<String>,
}

impl Issue {
    pub fn new(level: IssueLevel, message: impl Into<String>, location: Location) -> Self {
        Self {
            level,
            code: None,
            message: message.into(),
            location,
            context: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// 分析结果统计
#[derive(Debug, Default)]
pub struct AnalysisResult {
    pub total_issues: usize,
    pub issues_by_level: HashMap<IssueLevel, usize>,
    pub issues_by_type: HashMap<String, usize>,
    pub issues_by_file: HashMap<String, Vec<Issue>>,
    pub unique_patterns: HashSet<String>,
}

impl AnalysisResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_issues(issues: Vec<Issue>) -> Self {
        let mut result = Self::new();
        for issue in issues {
            result.add_issue(issue);
        }
        result
    }

    pub fn add_issue(&mut self, issue: Issue) {
        self.total_issues += 1;

        // 按级别统计
        *self.issues_by_level.entry(issue.level.clone()).or_insert(0) += 1;

        // 按类型统计（使用错误代码或消息模式）
        let type_key = issue
            .code
            .clone()
            .unwrap_or_else(|| self.extract_pattern(&issue.message));
        *self.issues_by_type.entry(type_key.clone()).or_insert(0) += 1;

        // 按文件统计
        self.issues_by_file
            .entry(issue.location.file_path.clone())
            .or_insert_with(Vec::new)
            .push(issue);

        // 记录唯一模式
        self.unique_patterns.insert(type_key);
    }

    fn extract_pattern(&self, message: &str) -> String {
        // 简化消息，提取模式
        // 移除具体的变量名、行号等
        message
            .split_whitespace()
            .take(5)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn errors(&self) -> Vec<&Issue> {
        self.issues_by_file
            .values()
            .flat_map(|issues| issues.iter())
            .filter(|i| i.level == IssueLevel::Error)
            .collect()
    }

    pub fn warnings(&self) -> Vec<&Issue> {
        self.issues_by_file
            .values()
            .flat_map(|issues| issues.iter())
            .filter(|i| i.level == IssueLevel::Warning)
            .collect()
    }
}

/// 技术栈类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechStack {
    Cargo,
    Maven,
    Npm,
    Pnpm,
    Yarn,
    Mypy,
    GoBuild,
    GolangciLint,
}

impl TechStack {
    pub fn as_str(&self) -> &'static str {
        match self {
            TechStack::Cargo => "cargo",
            TechStack::Maven => "maven",
            TechStack::Npm => "npm",
            TechStack::Pnpm => "pnpm",
            TechStack::Yarn => "yarn",
            TechStack::Mypy => "mypy",
            TechStack::GoBuild => "go",
            TechStack::GolangciLint => "golangci-lint",
        }
    }
}

impl std::str::FromStr for TechStack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cargo" | "rust" => Ok(TechStack::Cargo),
            "maven" | "mvn" | "java" => Ok(TechStack::Maven),
            "npm" | "node" => Ok(TechStack::Npm),
            "pnpm" => Ok(TechStack::Pnpm),
            "yarn" => Ok(TechStack::Yarn),
            "mypy" | "python" => Ok(TechStack::Mypy),
            "go" | "golang" => Ok(TechStack::GoBuild),
            "golangci-lint" => Ok(TechStack::GolangciLint),
            _ => Err(format!("Unknown tech stack: {}", s)),
        }
    }
}

/// 子命令类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubCommand {
    // Cargo 子命令
    Check,        // cargo check
    Clippy,       // cargo clippy
    ClippyAll,    // cargo clippy --all-targets --all-features
    CheckTest,    // cargo check --tests

    // Maven 子命令
    Compile,    // mvn compile
    MvnTest,    // mvn test

    // NPM 子命令
    Lint,       // npm run lint
    TypeCheck,  // npm run type-check
    Audit,      // npm audit

    // Mypy 子命令
    MypyCheck,       // mypy
    MypyCheckStrict, // mypy --strict

    // Go 子命令
    GoBuild,    // go build
    GoVet,      // go vet
    GoLint,     // golangci-lint
}

impl SubCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubCommand::Check => "check",
            SubCommand::Clippy => "clippy",
            SubCommand::ClippyAll => "clippy-all",
            SubCommand::CheckTest => "check-test",
            SubCommand::Compile => "compile",
            SubCommand::MvnTest => "test",
            SubCommand::Lint => "lint",
            SubCommand::TypeCheck => "type-check",
            SubCommand::Audit => "audit",
            SubCommand::MypyCheck => "check",
            SubCommand::MypyCheckStrict => "check-strict",
            SubCommand::GoBuild => "build",
            SubCommand::GoVet => "vet",
            SubCommand::GoLint => "lint",
        }
    }

    /// 获取该子命令的描述
    pub fn description(&self) -> &'static str {
        match self {
            SubCommand::Check => "Fast syntax and type checking",
            SubCommand::Clippy => "Run Clippy linter",
            SubCommand::ClippyAll => "Run Clippy on all targets and features",
            SubCommand::CheckTest => "Check test code syntax and types",
            SubCommand::Compile => "Compile the project",
            SubCommand::MvnTest => "Run tests",
            SubCommand::Lint => "Run linter",
            SubCommand::TypeCheck => "Run TypeScript type checker",
            SubCommand::Audit => "Audit dependencies for vulnerabilities",
            SubCommand::MypyCheck => "Run mypy type checker",
            SubCommand::MypyCheckStrict => "Run mypy in strict mode",
            SubCommand::GoBuild => "Build the project",
            SubCommand::GoVet => "Run go vet",
            SubCommand::GoLint => "Run golangci-lint",
        }
    }
}

impl std::str::FromStr for SubCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "check" => Ok(SubCommand::Check),
            "clippy" => Ok(SubCommand::Clippy),
            "clippy-all" => Ok(SubCommand::ClippyAll),
            "check-test" => Ok(SubCommand::CheckTest),
            "compile" => Ok(SubCommand::Compile),
            "lint" => Ok(SubCommand::Lint),
            "type-check" => Ok(SubCommand::TypeCheck),
            "audit" => Ok(SubCommand::Audit),
            "check-strict" => Ok(SubCommand::MypyCheckStrict),
            "build" => Ok(SubCommand::GoBuild),
            "vet" => Ok(SubCommand::GoVet),
            _ => Err(format!("Unknown subcommand: {}", s)),
        }
    }
}

/// 分析选项
#[derive(Debug, Default, Clone)]
pub struct AnalyzeOptions {
    pub subcommand: Option<SubCommand>,
    pub filter_warnings: bool,
    pub filter_paths: Vec<String>,
    pub output_file: Option<String>,
}

/// 报告格式
#[derive(Debug, Clone, Copy, Default)]
pub enum ReportFormat {
    #[default]
    Markdown,
    Json,
    Html,
}

impl std::str::FromStr for ReportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Ok(ReportFormat::Markdown),
            "json" => Ok(ReportFormat::Json),
            "html" => Ok(ReportFormat::Html),
            _ => Err(format!("Unknown report format: {}", s)),
        }
    }
}
