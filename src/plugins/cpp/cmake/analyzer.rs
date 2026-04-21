//! CMake Analyzer
//! Runs CMake configuration and build, then parses output

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, Config, SubCommand,
    OutputParser, TechStack,
};

use super::parser::CMakeParser;

pub struct CMakeAnalyzer {
    parser: CMakeParser,
    config: Option<Config>,
}

impl CMakeAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: CMakeParser::new(),
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
            if let Some(exec_str) = config.get_command_exec("cmake", command_name) {
                return CommandBuilder::from_exec_string(&exec_str);
            }
        }

        // Fallback to hardcoded commands
        let mut builder = CommandBuilder::new("cmake");

        // Get source directory (default to current directory)
        let source_dir = options.source_dir.as_deref().unwrap_or(".");
        // Get build directory (default to "build")
        let build_dir = options.build_dir.as_deref().unwrap_or("build");

        match options.subcommand {
            Some(SubCommand::Check) => {
                // Configure only, don't build
                builder = builder.arg("-B").arg(build_dir);
                builder = builder.arg("-S").arg(source_dir);
                // Can specify generator
                if let Some(ref generator) = options.cmake_generator {
                    builder = builder.arg("-G").arg(generator);
                }
            }
            _ => {
                // Build
                builder = builder.arg("--build").arg(build_dir);
                // Can specify target
                if let Some(ref target) = options.target {
                    builder = builder.arg("--target").arg(target);
                }
            }
        }

        builder
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

impl Default for CMakeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildAnalyzer for CMakeAnalyzer {
    fn tech_stack(&self) -> TechStack {
        TechStack::CMake
    }

    fn supported_commands(&self) -> Vec<&str> {
        vec!["cmake", "cmake-build", "cmake-check"]
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
