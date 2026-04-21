//! Mypy Analyzer
//Run mypy and parse the output. Run mypy and parse the output

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, Config, OutputParser,
    TechStack,
};

use super::parser::MypyParser;

pub struct MypyAnalyzer {
    parser: MypyParser,
    config: Option<Config>,
}

impl MypyAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: MypyParser::new(),
            config: None,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let command_name = options.subcommand.as_ref().map(|s| s.as_str()).unwrap_or("check");

        // Try to get command from config first
        if let Some(ref config) = self.config {
            if let Some(exec_str) = config.get_command_exec("mypy", command_name) {
                return CommandBuilder::from_exec_string(&exec_str);
            }
        }

        // Fallback to hardcoded commands
        let mut builder = CommandBuilder::new("mypy");

        // Check if the subcommand name indicates strict mode
        if let Some(ref cmd) = options.subcommand {
            if cmd.as_str() == "check-strict" {
                builder = builder.arg("--strict");
            }
        }

        builder.arg("--show-column-numbers").arg(".")
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

impl Default for MypyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildAnalyzer for MypyAnalyzer {
    fn tech_stack(&self) -> TechStack {
        TechStack::Mypy
    }

    fn supported_commands(&self) -> Vec<&str> {
        vec!["mypy", "python"]
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
