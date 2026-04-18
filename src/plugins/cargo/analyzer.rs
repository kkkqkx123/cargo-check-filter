//! Cargo 分析器
//! 运行 cargo check/clippy/test 并解析输出

use std::path::Path;

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, OutputParser,
    SubCommand,
};

use super::parser::CargoParser;

pub struct CargoAnalyzer {
    parser: CargoParser,
}

impl CargoAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: CargoParser::new(),
        }
    }

    fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("cargo");

        match options.subcommand {
            Some(SubCommand::Check) => {
                builder = builder.arg("check");
            }
            Some(SubCommand::Clippy) => {
                builder = builder.arg("clippy");
            }
            Some(SubCommand::ClippyAll) => {
                builder = builder
                    .arg("clippy")
                    .arg("--all-targets")
                    .arg("--all-features");
            }
            Some(SubCommand::CheckTest) => {
                builder = builder.arg("check").arg("--tests");
            }
            _ => {
                builder = builder.arg("check");
            }
        }

        builder.arg("--message-format=short")
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

impl Default for CargoAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildAnalyzer for CargoAnalyzer {
    fn name(&self) -> &str {
        "cargo"
    }

    fn supported_commands(&self) -> Vec<&str> {
        vec!["cargo", "rust"]
    }

    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("Cargo.toml").exists()
    }

    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let builder = self.create_command_builder(options);
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
