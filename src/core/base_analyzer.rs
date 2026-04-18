//! Implementation of the Basic Analyzer
//Provide general analyzer logic to reduce duplicate code

use super::analyzer::{AnalyzerError, BuildAnalyzer};
use super::types::{AnalysisResult, AnalyzeOptions, IssueLevel};
use super::command::CommandBuilder;

/// Basic Analyzer Trait
/// Provide general command execution and result filtering logic
pub trait BaseBuildAnalyzer: BuildAnalyzer {
    /// Build command
    fn build_command(&self, options: &AnalyzeOptions) -> Vec<String>;

    /// Run the command (default implementation)
    fn run_command(&self, cmd: &[String]) -> Result<String, AnalyzerError> {
        let program = &cmd[0];
        let args = &cmd[1..];

        println!("Running: {} {}", program, args.join(" "));

        let output = std::process::Command::new(program)
            .args(args)
            .output()
            .map_err(|e| AnalyzerError::CommandFailed(format!("Failed to execute {}: {}", program, e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("{}{}", stdout, stderr))
    }

    /// Filtering Issues (Default Implementation)
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
                if options.filter_warnings && matches!(issue.level, IssueLevel::Warning) {
                    continue;
                }

                filtered.add_issue(issue);
            }
        }

        filtered
    }

    /// Perform analysis (default implementation)
    fn execute_analysis(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let cmd = self.build_command(options);
        let output = self.run_command(&cmd)?;

        println!("Parsing output...");
        let issues = self.parser().parse(&output);
        println!("Found {} issues", issues.len());

        let result = AnalysisResult::from_issues(issues);
        Ok(self.filter_issues(result, options))
    }
}

/// Using the Analyzer trait with CommandBuilder
pub trait CommandBuilderAnalyzer: BuildAnalyzer {
    /// Create a command builder
    fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder;

    /// Run the command (using CommandBuilder)
    fn run_command_with_builder(&self, builder: &CommandBuilder) -> Result<String, AnalyzerError> {
        builder.execute()
    }

    /// Filtering Issues (Default Implementation)
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
                if options.filter_warnings && matches!(issue.level, IssueLevel::Warning) {
                    continue;
                }

                filtered.add_issue(issue);
            }
        }

        filtered
    }

    /// Perform analysis (using CommandBuilder)
    fn execute_analysis_with_builder(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let builder = self.create_command_builder(options);
        let output = self.run_command_with_builder(&builder)?;

        println!("Parsing output...");
        let issues = self.parser().parse(&output);
        println!("Found {} issues", issues.len());

        let result = AnalysisResult::from_issues(issues);
        Ok(self.filter_issues(result, options))
    }
}
