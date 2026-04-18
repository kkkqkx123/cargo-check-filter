//! Pytest Analyzer
//! Run pytest and parse the output

use std::path::Path;

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, OutputParser,
    ParsedTestOutput, SubCommand, TestAnalyzer, TestAnalyzerError, TestOptions, TestOutputParser,
};

use super::parser::PytestParser;

pub struct PytestAnalyzer {
    parser: PytestParser,
}

impl PytestAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: PytestParser::new(),
        }
    }

    /// Create command builder for pytest
    fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("pytest");

        // Add verbose flag for detailed output
        builder = builder.arg("-v");

        // Add color=no for easier parsing
        builder = builder.arg("--color=no");

        // Add tb=short for shorter traceback
        builder = builder.arg("--tb=short");

        match options.subcommand {
            Some(SubCommand::Pytest) => {
                // Default pytest run
            }
            Some(SubCommand::PytestQuiet) => {
                builder = builder.arg("-q");
            }
            Some(SubCommand::PytestVerbose) => {
                builder = builder.arg("-vv");
            }
            _ => {}
        }

        builder
    }

    /// Create test command builder
    fn create_test_command(&self, options: &TestOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("pytest");

        // Add verbose flag
        builder = builder.arg("-v");

        // Disable color for easier parsing
        builder = builder.arg("--color=no");

        // Add test filter if specified
        if let Some(ref filter) = options.filter {
            builder = builder.arg("-k").arg(filter);
        }

        // Run specific test file or directory if specified
        if let Some(ref test) = options.test {
            builder = builder.arg(test);
        }

        // Add extra arguments
        for arg in &options.extra_args {
            builder = builder.arg(arg);
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

impl Default for PytestAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildAnalyzer for PytestAnalyzer {
    fn name(&self) -> &str {
        "pytest"
    }

    fn supported_commands(&self) -> Vec<&str> {
        vec!["pytest", "py.test", "python-test"]
    }

    fn is_applicable(&self, project_path: &Path) -> bool {
        // Check for Python project indicators
        let has_python_project = project_path.join("requirements.txt").exists()
            || project_path.join("pyproject.toml").exists()
            || project_path.join("setup.py").exists()
            || project_path.join("setup.cfg").exists()
            || project_path.join("Pipfile").exists();

        // Check for test files or pytest configuration
        let has_test_files = project_path.join("tests").exists()
            || project_path.join("test").exists()
            || project_path.read_dir().map(|mut entries| {
                entries.any(|e| {
                    if let Ok(entry) = e {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        name_str.starts_with("test_") && name_str.ends_with(".py")
                            || name_str.ends_with("_test.py")
                    } else {
                        false
                    }
                })
            }).unwrap_or(false);

        let has_pytest_config = project_path.join("pytest.ini").exists()
            || project_path.join("pyproject.toml").exists()
            || project_path.join("tox.ini").exists()
            || project_path.join("setup.cfg").exists();

        has_python_project && (has_test_files || has_pytest_config)
    }

    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        // For pytest, "analyze" means running tests and reporting results
        let builder = self.create_command_builder(options);
        let output = builder.execute()?;

        println!("Parsing pytest output...");
        let parsed = self.parser.parse_test_output(&output);
        println!(
            "Found {} passed, {} failed, {} skipped",
            parsed.passed_tests.len(),
            parsed.failed_tests.len(),
            parsed.ignored_tests.len()
        );

        // Convert test failures to issues for the analysis result
        let mut result = AnalysisResult::new();

        // Add failed tests as issues
        for test in &parsed.failed_tests {
            if let Some(ref location) = test.location {
                let issue = crate::core::Issue::new(
                    crate::core::IssueLevel::Error,
                    format!("Test failed: {}", test.name),
                    location.clone(),
                )
                .with_context(test.failure_details.clone().unwrap_or_default());
                result.add_issue(issue);
            }
        }

        Ok(self.filter_issues(result, options))
    }

    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}

impl TestAnalyzer for PytestAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<ParsedTestOutput, TestAnalyzerError> {
        let builder = self.create_test_command(options);
        let output = builder
            .execute()
            .map_err(|e| TestAnalyzerError::CommandFailed(e.to_string()))?;

        // Parse test output
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pytest_analyzer_name() {
        let analyzer = PytestAnalyzer::new();
        assert_eq!(analyzer.name(), "pytest");
    }

    #[test]
    fn test_supported_commands() {
        let analyzer = PytestAnalyzer::new();
        let commands = analyzer.supported_commands();
        assert!(commands.contains(&"pytest"));
        assert!(commands.contains(&"py.test"));
        assert!(commands.contains(&"python-test"));
    }

    #[test]
    fn test_is_applicable_with_requirements() {
        let analyzer = PytestAnalyzer::new();
        // This test would need a temporary directory with the right files
        // For now, we just verify the logic exists
        assert!(!analyzer.is_applicable(Path::new("/nonexistent/path")));
    }
}
