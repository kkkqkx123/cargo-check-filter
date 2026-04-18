//! NPM/Node.js 分析器
//! 运行 npm/pnpm/yarn 命令并解析输出

use std::path::Path;

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, OutputParser,
    ParsedTestOutput, SubCommand, TestAnalyzer, TestAnalyzerError, TestOptions, TestOutputParser,
};

use super::parser::NpmParser;

/// Type-check 命令的候选名称（按优先级排序）
const TYPE_CHECK_ALIASES: &[&str] = &["type-check", "typecheck", "check-types", "check-type"];

/// Lint 命令的候选名称（按优先级排序）
const LINT_ALIASES: &[&str] = &["lint", "eslint", "lint:check"];

/// Test 命令的候选名称（按优先级排序）
/// 注意：这些是 package.json scripts 中的名称，不是测试框架名称
const TEST_ALIASES: &[&str] = &["test", "test:unit", "test:e2e", "unit-test"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    pub fn as_str(&self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
        }
    }

    pub fn detect(project_path: &Path) -> Option<PackageManager> {
        if project_path.join("pnpm-lock.yaml").exists() {
            Some(PackageManager::Pnpm)
        } else if project_path.join("yarn.lock").exists() {
            Some(PackageManager::Yarn)
        } else if project_path.join("package-lock.json").exists() {
            Some(PackageManager::Npm)
        } else {
            Some(PackageManager::Npm)
        }
    }

    fn build_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
        match self {
            PackageManager::Npm => self.build_npm_command(options),
            PackageManager::Pnpm => self.build_pnpm_command(options),
            PackageManager::Yarn => self.build_yarn_command(options),
        }
    }

    fn build_npm_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("npm");

        match options.subcommand {
            Some(SubCommand::Lint) => {
                builder = builder.arg("run").arg(LINT_ALIASES[0]);
            }
            Some(SubCommand::TypeCheck) => {
                builder = builder.arg("run").arg(TYPE_CHECK_ALIASES[0]);
            }
            Some(SubCommand::Audit) => {
                builder = builder.arg("audit");
            }
            _ => {
                builder = builder.arg("run").arg(LINT_ALIASES[0]);
            }
        }

        builder
    }

    fn build_pnpm_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("pnpm");

        match options.subcommand {
            Some(SubCommand::Lint) => {
                builder = builder.arg(LINT_ALIASES[0]);
            }
            Some(SubCommand::TypeCheck) => {
                builder = builder.arg(TYPE_CHECK_ALIASES[0]);
            }
            Some(SubCommand::Audit) => {
                builder = builder.arg("audit");
            }
            _ => {
                builder = builder.arg(LINT_ALIASES[0]);
            }
        }

        builder
    }

    fn build_yarn_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("yarn");

        match options.subcommand {
            Some(SubCommand::Lint) => {
                builder = builder.arg(LINT_ALIASES[0]);
            }
            Some(SubCommand::TypeCheck) => {
                builder = builder.arg(TYPE_CHECK_ALIASES[0]);
            }
            Some(SubCommand::Audit) => {
                builder = builder.arg("audit");
            }
            _ => {
                builder = builder.arg(LINT_ALIASES[0]);
            }
        }

        builder
    }
}

pub struct NpmAnalyzer {
    parser: NpmParser,
    package_manager: PackageManager,
}

impl NpmAnalyzer {
    pub fn new(package_manager: PackageManager) -> Self {
        Self {
            parser: NpmParser::new(),
            package_manager,
        }
    }

    pub fn npm() -> Self {
        Self::new(PackageManager::Npm)
    }

    pub fn pnpm() -> Self {
        Self::new(PackageManager::Pnpm)
    }

    pub fn yarn() -> Self {
        Self::new(PackageManager::Yarn)
    }

    fn filter_issues(&self, result: AnalysisResult, options: &AnalyzeOptions) -> AnalysisResult {
        if !options.filter_warnings && options.filter_paths.is_empty() {
            return result;
        }

        let mut filtered = AnalysisResult::new();

        for (file_path, issues) in result.issues_by_file {
            if !options.filter_paths.is_empty() {
                let matches = options
                    .filter_paths
                    .iter()
                    .any(|filter| file_path.contains(filter));
                if !matches {
                    continue;
                }
            }

            for issue in issues {
                if options.filter_warnings && matches!(issue.level, crate::core::IssueLevel::Warning)
                {
                    continue;
                }

                filtered.add_issue(issue);
            }
        }

        filtered
    }

    /// 创建测试命令
    fn create_test_command(&self, options: &TestOptions) -> CommandBuilder {
        let script_name = Self::find_script_name(TEST_ALIASES);
        
        let mut builder = match self.package_manager {
            PackageManager::Npm => {
                CommandBuilder::new("npm").arg("run").arg(script_name)
            }
            PackageManager::Pnpm => {
                CommandBuilder::new("pnpm").arg(script_name)
            }
            PackageManager::Yarn => {
                CommandBuilder::new("yarn").arg(script_name)
            }
        };

        // 添加测试过滤器（测试名称模式）
        if let Some(ref filter) = options.filter {
            builder = builder.arg(filter);
        }

        builder
    }

    /// 查找脚本名称（从候选列表中返回第一个）
    fn find_script_name<'a>(candidates: &'a [&str]) -> &'a str {
        candidates.first().copied().unwrap_or(candidates[0])
    }
}

impl BuildAnalyzer for NpmAnalyzer {
    fn name(&self) -> &str {
        self.package_manager.as_str()
    }

    fn supported_commands(&self) -> Vec<&str> {
        match self.package_manager {
            PackageManager::Npm => vec!["npm", "node"],
            PackageManager::Pnpm => vec!["pnpm"],
            PackageManager::Yarn => vec!["yarn"],
        }
    }

    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("package.json").exists()
    }

    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let builder = self.package_manager.build_command(options);
        let output = builder.execute()?;

        println!("Parsing output...");
        let issues = self.parser.parse(&output);
        println!("Found {} issues", issues.len());

        let result = AnalysisResult::from_issues(issues);
        Ok(self.filter_issues(result, options))
    }

    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}

impl TestAnalyzer for NpmAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<ParsedTestOutput, TestAnalyzerError> {
        let builder = self.create_test_command(options);
        let output = builder
            .execute()
            .map_err(|e| TestAnalyzerError::CommandFailed(e.to_string()))?;

        // 使用 TestOutputParser 解析输出
        let parsed = self
            .test_parser()
            .ok_or(TestAnalyzerError::NotSupported)?
            .parse_test_output(&output);

        Ok(parsed)
    }

    fn test_parser(&self) -> Option<&dyn TestOutputParser> {
        Some(&self.parser)
    }
}
