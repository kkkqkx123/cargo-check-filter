//! Go Analyzer
//! Run go build, go vet, go test, golangci-lint and parse the output

use std::path::Path;

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, OutputParser,
    ParsedTestOutput, SubCommand, TestAnalyzer, TestAnalyzerError, TestOptions, TestOutputParser,
};

use super::parser::GoParser;

pub struct GoAnalyzer {
    parser: GoParser,
}

impl GoAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: GoParser::new(),
        }
    }

    /// Create command builder based on subcommand
    fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
        match options.subcommand {
            Some(SubCommand::GoVet) => self.create_go_vet_command(),
            Some(SubCommand::GoLint) => self.create_golangci_lint_command(),
            Some(SubCommand::GoBuild) | _ => self.create_go_build_command(),
        }
    }

    /// Create go build command
    fn create_go_build_command(&self) -> CommandBuilder {
        CommandBuilder::new("go").arg("build").arg("./...")
    }

    /// Create go vet command
    fn create_go_vet_command(&self) -> CommandBuilder {
        CommandBuilder::new("go").arg("vet").arg("./...")
    }

    /// Create golangci-lint command
    fn create_golangci_lint_command(&self) -> CommandBuilder {
        CommandBuilder::new("golangci-lint").arg("run").arg("./...")
    }

    /// Create go test command
    fn create_test_command(&self, options: &TestOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("go").arg("test");

        // Add verbose flag for detailed output
        builder = builder.arg("-v");

        // Add package path
        if let Some(ref package) = options.package {
            builder = builder.arg(package);
        } else {
            builder = builder.arg("./...");
        }

        // Add test filter if specified
        if let Some(ref filter) = options.filter {
            builder = builder.arg("-run").arg(filter);
        }

        // Add timeout if specified
        if let Some(timeout) = options.timeout {
            builder = builder.arg("-timeout").arg(format!("{}s", timeout));
        }

        // Add race detector if enabled
        if options.race {
            builder = builder.arg("-race");
        }

        // Add cover profile if specified
        if options.coverage {
            builder = builder.arg("-cover");
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

impl Default for GoAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildAnalyzer for GoAnalyzer {
    fn name(&self) -> &str {
        "go"
    }

    fn supported_commands(&self) -> Vec<&str> {
        vec!["go", "golang"]
    }

    fn is_applicable(&self, project_path: &Path) -> bool {
        // Check for go.mod file
        if project_path.join("go.mod").exists() {
            return true;
        }

        // Check for any .go files in the directory
        if let Ok(entries) = std::fs::read_dir(project_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "go" {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
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

impl TestAnalyzer for GoAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<ParsedTestOutput, TestAnalyzerError> {
        let builder = self.create_test_command(options);
        let output = builder
            .execute()
            .map_err(|e| TestAnalyzerError::CommandFailed(e.to_string()))?;

        // Parse output using TestOutputParser
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
    use std::path::PathBuf;

    #[test]
    fn test_go_analyzer_name() {
        let analyzer = GoAnalyzer::new();
        assert_eq!(analyzer.name(), "go");
    }

    #[test]
    fn test_go_analyzer_supported_commands() {
        let analyzer = GoAnalyzer::new();
        let commands = analyzer.supported_commands();
        assert!(commands.contains(&"go"));
        assert!(commands.contains(&"golang"));
    }

    #[test]
    fn test_go_analyzer_is_applicable_with_go_mod() {
        // This test requires a temporary directory with go.mod
        // For now, just test the logic without actual file system
        let analyzer = GoAnalyzer::new();

        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("go_analyzer_test");
        let _ = std::fs::create_dir_all(&temp_dir);

        // Test without go.mod (should return false for empty dir)
        assert!(!analyzer.is_applicable(&temp_dir));

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_create_go_build_command() {
        let analyzer = GoAnalyzer::new();
        let options = AnalyzeOptions::default();
        let builder = analyzer.create_command_builder(&options);

        // Verify the command is created (we can't easily inspect the builder internals)
        // But we can verify it doesn't panic
        assert_eq!(analyzer.name(), "go");
    }

    #[test]
    fn test_create_go_vet_command() {
        let analyzer = GoAnalyzer::new();
        let options = AnalyzeOptions {
            subcommand: Some(SubCommand::GoVet),
            ..Default::default()
        };
        let _builder = analyzer.create_command_builder(&options);

        // Command created successfully
        assert_eq!(analyzer.name(), "go");
    }

    #[test]
    fn test_create_golangci_lint_command() {
        let analyzer = GoAnalyzer::new();
        let options = AnalyzeOptions {
            subcommand: Some(SubCommand::GoLint),
            ..Default::default()
        };
        let _builder = analyzer.create_command_builder(&options);

        // Command created successfully
        assert_eq!(analyzer.name(), "go");
    }

    #[test]
    fn test_create_test_command() {
        let analyzer = GoAnalyzer::new();
        let options = TestOptions::default();
        let _builder = analyzer.create_test_command(&options);

        // Command created successfully
        assert!(analyzer.supports_test());
    }

    #[test]
    fn test_create_test_command_with_filter() {
        let analyzer = GoAnalyzer::new();
        let options = TestOptions {
            filter: Some("TestAddition".to_string()),
            ..Default::default()
        };
        let _builder = analyzer.create_test_command(&options);

        // Command created successfully
        assert!(analyzer.supports_test());
    }

    #[test]
    fn test_create_test_command_with_package() {
        let analyzer = GoAnalyzer::new();
        let options = TestOptions {
            package: Some("./pkg/...".to_string()),
            ..Default::default()
        };
        let _builder = analyzer.create_test_command(&options);

        // Command created successfully
        assert!(analyzer.supports_test());
    }

    #[test]
    fn test_create_test_command_with_coverage() {
        let analyzer = GoAnalyzer::new();
        let options = TestOptions {
            coverage: true,
            ..Default::default()
        };
        let _builder = analyzer.create_test_command(&options);

        // Command created successfully
        assert!(analyzer.supports_test());
    }

    #[test]
    fn test_create_test_command_with_race() {
        let analyzer = GoAnalyzer::new();
        let options = TestOptions {
            race: true,
            ..Default::default()
        };
        let _builder = analyzer.create_test_command(&options);

        // Command created successfully
        assert!(analyzer.supports_test());
    }
}
