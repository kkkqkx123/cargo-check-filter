//! Analyzer - Multilingual Build Tool Error Analyzer
//!
//! Usage: analyzer <tech-stack> <subcommand> [options]
//!
//! Examples:
//!   analyzer cargo check
//!   analyzer cargo clippy-all
//!   analyzer cargo test --filter-warnings

use std::env;
use std::path::Path;

mod core;
mod plugins;

use core::{
    AnalysisResult, AnalyzeOptions, ReportFormat, ReporterFactory, SubCommand, TechStack,
    TestAnalyzer, TestOptions, Config,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        std::process::exit(1);
    }

    // Load configuration
    let config = Config::load(Path::new(".")).unwrap_or_default();

    // Parse arguments
    let (tech_stack, options) = parse_arguments(&args, &config);

    // Creating a plug-in registry
    let registry = plugins::create_registry();

    // Get the corresponding analyzer
    let analyzer = match registry.get(tech_stack) {
        Some(a) => a,
        None => {
            eprintln!("Error: Unknown tech stack '{}'", tech_stack.as_str());
            eprintln!("Supported: {}", registry.list().join(", "));
            std::process::exit(1);
        }
    };

    // Check if analyzer is applicable for this project
    if let Err(e) = registry.check_applicable(tech_stack, Path::new(".")) {
        eprintln!("Warning: {}", e);
    }

    // Print supported commands for this analyzer
    println!("Supported commands: {}", analyzer.supported_commands().join(", "));

    // Check if this is a test command
    let is_test_command = is_test_subcommand(&options.subcommand);

    // Print subcommand category if available
    if let Some(ref cmd) = options.subcommand {
        println!("Command category: {:?}", cmd.category());
        if cmd.is_custom() {
            println!("Using custom command: {}", cmd.as_str());
        }
    }

    if is_test_command {
        // Run test analysis
        run_test_analysis(analyzer, &options, &config);
    } else {
        // Run regular analysis
        run_analysis(analyzer, &options, &config);
    }
}

fn parse_arguments(args: &[String], config: &Config) -> (TechStack, AnalyzeOptions) {
    let mut tech_stack_str = String::new();
    let mut subcommand: Option<SubCommand> = None;
    let mut options = AnalyzeOptions::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                show_help();
                std::process::exit(0);
            }
            "--version" | "-v" => {
                println!("analyzer 0.2.0");
                std::process::exit(0);
            }
            "--filter-warnings" => {
                options.filter_warnings = true;
            }
            "--filter-paths" => {
                if i + 1 < args.len() {
                    options.filter_paths = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    i += 1;
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    options.output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            arg => {
                if !arg.starts_with('-') {
                    if tech_stack_str.is_empty() {
                        tech_stack_str = arg.to_string();
                    } else if subcommand.is_none() {
                        // Parse subcommand
                        match arg.parse::<SubCommand>() {
                            Ok(cmd) => subcommand = Some(cmd),
                            Err(_) => {
                                eprintln!("Error: Unknown subcommand '{}'", arg);
                                eprintln!("Run 'analyzer --help' for usage information.");
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }

    if tech_stack_str.is_empty() {
        eprintln!("Error: No tech stack specified");
        show_help();
        std::process::exit(1);
    }

    // Parse tech stack
    let tech_stack: TechStack = tech_stack_str.parse().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    // If no subcommand is specified, use default based on tech stack
    if subcommand.is_none() {
        subcommand = Some(get_default_subcommand(tech_stack));
    }

    // Apply config defaults if not overridden by CLI
    if !options.filter_warnings && config.global.filter_warnings {
        options.filter_warnings = true;
    }

    options.subcommand = subcommand;
    (tech_stack, options)
}

fn is_test_subcommand(subcommand: &Option<SubCommand>) -> bool {
    matches!(
        subcommand,
        Some(SubCommand::CheckTest) | Some(SubCommand::Test)
    )
}

fn get_default_subcommand(tech_stack: TechStack) -> SubCommand {
    match tech_stack {
        TechStack::Cargo => SubCommand::Check,
        TechStack::Maven | TechStack::Gradle => SubCommand::Compile,
        TechStack::Npm | TechStack::Pnpm | TechStack::Yarn => SubCommand::Lint,
        TechStack::Mypy => SubCommand::TypeCheck,
        TechStack::Pytest => SubCommand::Test,
        TechStack::GoBuild | TechStack::GolangciLint => SubCommand::Build,
        TechStack::CMake => SubCommand::Check,
        TechStack::Gcc => SubCommand::Check,
        TechStack::Clang => SubCommand::Check,
        TechStack::Msvc => SubCommand::Check,
    }
}

fn run_analysis(
    analyzer: &dyn core::BuildAnalyzer,
    options: &AnalyzeOptions,
    _config: &Config,
) {
    let subcommand_name = options.subcommand.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("default");
    println!("Analyzing project with {} {}...", analyzer.name(), subcommand_name);

    // Use the parser method to demonstrate it's being used
    let _parser = analyzer.parser();
    println!("Using parser: {}", std::any::type_name_of_val(_parser));

    // The StreamingOutputParser trait is implemented by various parsers
    // and provides streaming/line-by-line parsing capabilities

    match analyzer.analyze(options) {
        Ok(result) => {
            println!("\nAnalysis complete!");
            println!("Total issues: {}", result.total_issues);

            // Generating reports
            let reporter = ReporterFactory::create(ReportFormat::Markdown);
            let report = match reporter.generate(&result) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to generate report: {}", e);
                    std::process::exit(1);
                }
            };

            // output report
            let output_path = options
                .output_file
                .as_deref()
                .unwrap_or("analysis_report.md");

            if let Err(e) = reporter.write_to_file(&report, Path::new(output_path)) {
                eprintln!("Failed to write report: {}", e);
                std::process::exit(1);
            }

            println!("Report written to: {}", output_path);

            // Print summary
            print_summary(&result);
        }
        Err(e) => {
            eprintln!("Analysis failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_test_analysis(
    analyzer: &dyn core::BuildAnalyzer,
    options: &AnalyzeOptions,
    _config: &Config,
) {
    // Try to downcast to TestAnalyzer
    let test_analyzer = match analyzer.as_any().downcast_ref::<&dyn TestAnalyzer>() {
        Some(ta) => *ta,
        None => {
            eprintln!("Error: Test analysis not supported for {}", analyzer.name());
            std::process::exit(1);
        }
    };

    if !test_analyzer.supports_test() {
        eprintln!("Error: Test analysis not supported for {}", analyzer.name());
        std::process::exit(1);
    }

    println!("Running tests for {}...", analyzer.name());

    // Convert AnalyzeOptions to TestOptions
    let test_options = TestOptions::from(options);

    match test_analyzer.run_tests(&test_options) {
        Ok(test_output) => {
            println!("\nTest analysis complete!");
            println!("Compile issues: {}", test_output.compile_issues.len());

            if let Some(ref summary) = test_output.test_summary {
                println!("Tests: {} total, {} passed, {} failed, {} ignored",
                    summary.total, summary.passed, summary.failed, summary.ignored);
            }

            // Generate test report
            let reporter = ReporterFactory::create(ReportFormat::Markdown);
            let report = match reporter.generate_test_report(&test_output.into()) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to generate report: {}", e);
                    std::process::exit(1);
                }
            };

            let output_path = options
                .output_file
                .as_deref()
                .unwrap_or("test_report.md");

            if let Err(e) = reporter.write_to_file(&report, Path::new(output_path)) {
                eprintln!("Failed to write report: {}", e);
                std::process::exit(1);
            }

            println!("Test report written to: {}", output_path);
        }
        Err(e) => {
            eprintln!("Test analysis failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn show_help() {
    println!("analyzer - Multi-language build tool error analyzer");
    println!();
    println!("Usage: analyzer <tech-stack> <subcommand> [options]");
    println!();
    println!("Tech Stacks:");
    println!("  cargo         Rust/Cargo projects");
    println!("  mypy          Python/Mypy projects");
    println!("  npm           Node.js/npm projects");
    println!("  pnpm          Node.js/pnpm projects");
    println!("  yarn          Node.js/yarn projects");
    println!("  go            Go projects");
    println!("  maven         Java/Maven projects");
    println!();
    println!("Subcommands for Cargo:");
    println!("  check         Fast syntax and type checking (cargo check)");
    println!("  clippy        Run Clippy linter (cargo clippy)");
    println!("  clippy-all    Run Clippy on all targets and features");
    println!("  test          Run tests and analyze output (cargo test)");
    println!();
    println!("Subcommands for Mypy:");
    println!("  check         Run mypy type checker");
    println!("  check-strict  Run mypy in strict mode");
    println!();
    println!("Subcommands for NPM/PNPM/Yarn:");
    println!("  lint          Run linter (npm run lint / pnpm lint / yarn lint)");
    println!("  type-check    Run TypeScript type checker");
    println!("  audit         Audit dependencies for vulnerabilities");
    println!();
    println!("Global Options:");
    println!("  -h, --help              Show this help message");
    println!("  -v, --version           Show version");
    println!("  --filter-warnings       Filter out warnings, show only errors");
    println!("  --filter-paths <paths>  Filter by file paths (comma-separated)");
    println!("  -o, --output <file>     Output file (default: analysis_report.md)");
    println!();
    println!("Examples:");
    println!("  analyzer cargo check");
    println!("  analyzer cargo clippy-all");
    println!("  analyzer cargo test --filter-warnings");
    println!("  analyzer cargo check --filter-paths src/core,src/lib -o report.md");
    println!("  analyzer mypy check");
    println!("  analyzer mypy check-strict");
    println!("  analyzer npm lint");
    println!("  analyzer pnpm type-check");
    println!("  analyzer yarn audit");
}

fn print_summary(result: &AnalysisResult) {
    println!("\n=== Summary ===");
    println!("Total issues: {}", result.total_issues);

    // Use error_count() and warning_count() methods
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());

    // Use errors() and warnings() methods for detailed counts
    let errors = result.errors();
    let warnings = result.warnings();
    println!("  (via errors() method: {})", errors.len());
    println!("  (via warnings() method: {})", warnings.len());

    for (level, count) in &result.issues_by_level {
        println!("  {}s: {}", level, count);
    }

    if !result.issues_by_file.is_empty() {
        println!("\nTop files with issues:");
        let mut files: Vec<_> = result.issues_by_file.iter().collect();
        files.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (file, issues) in files.iter().take(5) {
            println!("  {}: {} issues", file, issues.len());
        }
    }

    // Print first few errors if any
    if !errors.is_empty() {
        println!("\nFirst {} error(s):", std::cmp::min(3, errors.len()));
        for error in errors.iter().take(3) {
            println!("  - [{}] {}", error.location.file_path, error.message);
        }
    }
}
