//! 测试分析器 trait 定义
//! 定义测试执行的统一接口

use super::types::{Issue, TestCase, TestSummary};

/// 测试分析器错误
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

/// 解析后的测试输出
#[derive(Debug, Default)]
pub struct ParsedTestOutput {
    /// 编译阶段的问题
    pub compile_issues: Vec<Issue>,
    /// 测试摘要
    pub test_summary: Option<TestSummary>,
    /// 失败的测试用例
    pub failed_tests: Vec<TestCase>,
    /// 通过的测试用例
    pub passed_tests: Vec<TestCase>,
    /// 被忽略的测试用例
    pub ignored_tests: Vec<TestCase>,
}

impl ParsedTestOutput {
    pub fn new() -> Self {
        Self::default()
    }
}

/// 测试输出解析器 trait
pub trait TestOutputParser: Send + Sync {
    /// 解析测试输出
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput;
}

/// 测试选项
#[derive(Debug, Default, Clone)]
pub struct TestOptions {
    /// 测试过滤器（如 test name pattern）
    pub filter: Option<String>,
    /// 仅运行库测试
    pub lib_only: bool,
    /// 仅运行指定二进制文件的测试
    pub bin: Option<String>,
    /// 仅运行集成测试
    pub test: Option<String>,
    /// 仅运行文档测试
    pub doc_only: bool,
    /// 其他参数
    pub extra_args: Vec<String>,
}

/// 测试分析器 trait
/// 实现此 trait 以支持测试执行和分析
pub trait TestAnalyzer: Send + Sync {
    /// 是否支持测试分析
    fn supports_test(&self) -> bool;

    /// 运行测试并返回解析后的输出
    fn run_tests(&self, options: &TestOptions) -> Result<ParsedTestOutput, TestAnalyzerError>;

    /// 获取测试解析器
    fn test_parser(&self) -> Option<&dyn TestOutputParser> {
        None
    }
}
