//! Test analyzer trait definition
//! Define a uniform interface for test execution

use super::types::{Issue, TestCase, TestSummary};

/// Test Analyzer Error
#[derive(Debug)]
pub enum TestAnalyzerError {
    CommandFailed(String),
    ParseError(String),
    NotSupported,
}

impl std::fmt::Display for TestAnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestAnalyzerError::CommandFailed(msg) => write!(f, "Test command failed: {}", msg),
            TestAnalyzerError::ParseError(msg) => write!(f, "Test parse error: {}", msg),
            TestAnalyzerError::NotSupported => {
                write!(f, "Test analysis not supported for this analyzer")
            }
        }
    }
}

impl std::error::Error for TestAnalyzerError {}

/// Parsed test output
#[derive(Debug, Default)]
pub struct ParsedTestOutput {
    /// Problems at the compilation stage
    pub compile_issues: Vec<Issue>,
    /// Test Summary
    pub test_summary: Option<TestSummary>,
    /// Failed Test Cases
    pub failed_tests: Vec<TestCase>,
    /// Test cases passed
    pub passed_tests: Vec<TestCase>,
    /// Neglected Test Cases
    pub ignored_tests: Vec<TestCase>,
}

impl ParsedTestOutput {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Test output parser trait
pub trait TestOutputParser: Send + Sync {
    /// Parsing Test Output
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput;
}

/// Test Options
#[derive(Debug, Default, Clone)]
pub struct TestOptions {
    /// Test filters (e.g. test name pattern)
    pub filter: Option<String>,
    /// Run library tests only
    pub lib_only: bool,
    /// Run only the tests for the specified binary
    pub bin: Option<String>,
    /// Running Integration Tests Only
    pub test: Option<String>,
    /// Running Documentation Tests Only
    pub doc_only: bool,
    /// Package path (for Go: ./..., ./pkg/...)
    pub package: Option<String>,
    /// Test timeout in seconds
    pub timeout: Option<u64>,
    /// Enable race detector
    pub race: bool,
    /// Enable coverage reporting
    pub coverage: bool,
    /// Other parameters
    pub extra_args: Vec<String>,
}

/// Test Analyzer trait
/// Implement this trait to support test execution and analysis
pub trait TestAnalyzer: Send + Sync {
    /// Whether to support test analysis
    fn supports_test(&self) -> bool;

    /// Run the test and return the parsed output
    fn run_tests(&self, options: &TestOptions) -> Result<ParsedTestOutput, TestAnalyzerError>;

    /// Getting the test parser
    fn test_parser(&self) -> Option<&dyn TestOutputParser> {
        None
    }
}
