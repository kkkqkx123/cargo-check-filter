//! Analyzer - Multilingual Build Tool Error Analyzer
//!
//! 用法: analyzer <tech-stack> <subcommand> [options]
//!
//! Example.
//!   analyzer cargo check
//!   analyzer cargo clippy-all
//!   analyzer cargo test --filter-warnings

use std::env;
use std::path::Path;

mod core;
mod plugins;

use core::{
    AnalyzeOptions, ReportFormat, ReporterFactory, SubCommand,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        std::process::exit(1);
    }

    // parameterization
    let (tech_stack, options) = parse_arguments(&args);

    // Creating a plug-in registry
    let registry = plugins::create_registry();

    // Get the corresponding analyzer
    let analyzer = match registry.get(&tech_stack) {
        Some(a) => a,
        None => {
            eprintln!("Error: Unknown tech stack '{}'", tech_stack);
            eprintln!("Supported: {}", registry.list().join(", "));
            std::process::exit(1);
        }
    };

    // Check for applicability
    let current_dir = env::current_dir().expect("Failed to get current directory");
    if !analyzer.is_applicable(&current_dir) {
        eprintln!(
            "Error: '{}' is not applicable to this project",
            tech_stack
        );
        std::process::exit(1);
    }

    // operational analysis
    let subcommand_name = options.subcommand.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("default");
    println!("Analyzing project with {} {}...", analyzer.name(), subcommand_name);

    match analyzer.analyze(&options) {
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

fn parse_arguments(args: &[String]) -> (String, AnalyzeOptions) {
    let mut tech_stack = String::new();
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
                    if tech_stack.is_empty() {
                        tech_stack = arg.to_string();
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

    if tech_stack.is_empty() {
        eprintln!("Error: No tech stack specified");
        show_help();
        std::process::exit(1);
    }

    // If no subcommand is specified, the default value is used
    if subcommand.is_none() {
        subcommand = Some(get_default_subcommand(&tech_stack));
    }

    options.subcommand = subcommand;
    (tech_stack, options)
}

/// Get default subcommands based on tech stack
fn get_default_subcommand(tech_stack: &str) -> SubCommand {
    match tech_stack.to_lowercase().as_str() {
        "cargo" | "rust" => SubCommand::Check,
        "maven" | "mvn" => SubCommand::Compile,
        "npm" | "pnpm" | "yarn" => SubCommand::Lint,
        "mypy" | "python" => SubCommand::MypyCheck,
        "go" | "golang" => SubCommand::GoBuild,
        _ => SubCommand::Check,
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

fn print_summary(result: &core::AnalysisResult) {
    println!("\n=== Summary ===");
    println!("Total issues: {}", result.total_issues);

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
}
