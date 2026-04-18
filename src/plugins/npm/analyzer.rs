//! NPM/Node.js 分析器
//! 运行 npm/pnpm/yarn 命令并解析输出

use std::path::Path;

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, OutputParser,
    SubCommand,
};

use super::parser::NpmParser;

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
                builder = builder.arg("run").arg("lint");
            }
            Some(SubCommand::TypeCheck) => {
                builder = builder.arg("run").arg("type-check");
            }
            Some(SubCommand::Audit) => {
                builder = builder.arg("audit");
            }
            _ => {
                builder = builder.arg("run").arg("lint");
            }
        }

        builder
    }

    fn build_pnpm_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("pnpm");

        match options.subcommand {
            Some(SubCommand::Lint) => {
                builder = builder.arg("lint");
            }
            Some(SubCommand::TypeCheck) => {
                builder = builder.arg("type-check");
            }
            Some(SubCommand::Audit) => {
                builder = builder.arg("audit");
            }
            _ => {
                builder = builder.arg("lint");
            }
        }

        builder
    }

    fn build_yarn_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("yarn");

        match options.subcommand {
            Some(SubCommand::Lint) => {
                builder = builder.arg("lint");
            }
            Some(SubCommand::TypeCheck) => {
                builder = builder.arg("type-check");
            }
            Some(SubCommand::Audit) => {
                builder = builder.arg("audit");
            }
            _ => {
                builder = builder.arg("lint");
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
