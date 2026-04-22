//! NPM/Node.js Analyzer
//! Run the npm/pnpm/yarn command and parse the output

use crate::core::{
    AnalysisResult, AnalyzeOptions, AnalyzerError, BuildAnalyzer, CommandBuilder, Config, OutputParser,
    ParsedTestOutput, SubCommand, TechStack, TestAnalyzer, TestAnalyzerError, TestOptions,
    TestOutputParser,
};

use super::parser::NpmParser;

/// Candidate names for the Type-check command (in order of priority)
const TYPE_CHECK_ALIASES: &[&str] = &["type-check", "typecheck", "check-types", "check-type"];

/// Candidate names for Lint commands (in order of priority)
const LINT_ALIASES: &[&str] = &["lint", "eslint", "lint:check"];

/// Candidate names for the Test command (in order of priority)
/// Note: These are the names in the package.json scripts, not the test framework names.
const TEST_ALIASES: &[&str] = &["test", "test:unit", "test:e2e", "unit-test"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    fn as_str(&self) -> &str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
        }
    }

    fn build_command(&self, options: &AnalyzeOptions, config: &Option<Config>) -> CommandBuilder {
        let command_name = options.subcommand.as_ref().map(|s| s.as_str()).unwrap_or("lint");

        // Try to get command from config first
        if let Some(ref cfg) = config {
            if let Some(exec_str) = cfg.get_command_exec(self.as_str(), command_name) {
                return CommandBuilder::from_exec_string(&exec_str);
            }
        }

        // Fallback to hardcoded commands
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
    config: Option<Config>,
}

impl NpmAnalyzer {
    pub fn new(package_manager: PackageManager) -> Self {
        Self {
            parser: NpmParser::new(),
            package_manager,
            config: None,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
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

    /// Creating a test command
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

        // Adding test filters (test name mode)
        if let Some(ref filter) = options.filter {
            builder = builder.arg(filter);
        }

        builder
    }

    /// Find Script Name (returns the first one from the candidate list)
    fn find_script_name<'a>(candidates: &'a [&str]) -> &'a str {
        candidates.first().copied().unwrap_or(candidates[0])
    }
}

impl BuildAnalyzer for NpmAnalyzer {
    fn tech_stack(&self) -> TechStack {
        match self.package_manager {
            PackageManager::Npm => TechStack::Npm,
            PackageManager::Pnpm => TechStack::Pnpm,
            PackageManager::Yarn => TechStack::Yarn,
        }
    }

    fn supported_commands(&self) -> Vec<&str> {
        match self.package_manager {
            PackageManager::Npm => vec!["npm", "node"],
            PackageManager::Pnpm => vec!["pnpm"],
            PackageManager::Yarn => vec!["yarn"],
        }
    }

    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let builder = self.package_manager.build_command(options, &self.config);
        let output = builder.execute_with_status()?;

        println!("Parsing output...");
        let issues = self.parser.parse(&output.combined);
        println!("Found {} issues", issues.len());

        // If command failed but no issues were found, output the full raw output
        // to help diagnose parsing or environment issues
        if !output.success() && issues.is_empty() {
            eprintln!("\n=== Command failed with exit code {:?} but no issues were parsed ===", output.code());
            eprintln!("=== Raw output (stdout + stderr) ===");
            eprintln!("{}", output.combined);
            eprintln!("=== End of raw output ===\n");
        }

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

impl TestAnalyzer for NpmAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<ParsedTestOutput, TestAnalyzerError> {
        let builder = self.create_test_command(options);
        let output = builder
            .execute()
            .map_err(|e| TestAnalyzerError::CommandFailed(e.to_string()))?;

        // Parsing Output with TestOutputParser
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
