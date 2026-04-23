---
name: analyzer-usage
description: "Analyzes build tool errors from Cargo, NPM, Mypy, Maven, Gradle, Go, Pytest, and C++ (CMake/GCC/Clang/MSVC). Invoke when user asks to analyze build errors, check code quality issues, or use the analyzer CLI tool."
---

# Analyzer - Multi-language Build Tool Error Analyzer

This skill provides guidance on using the analyzer binary to analyze errors from various build tools and generate reports.

## Quick Start

```bash
# Run analyzer
analyzer <tech-stack> <subcommand> [options]
```

## Supported Tech Stacks

| Tech Stack      | Commands                      |
| --------------- | ----------------------------- |
| Cargo (Rust)    | `check`, `clippy`, `test`     |
| Mypy (Python)   | `check`, `check-strict`       |
| NPM (Node.js)   | `lint`, `type-check`, `audit` |
| Maven (Java)    | `compile`, `test`             |
| Gradle (Java)   | `compileJava`, `test`         |
| Go              | `build`, `test`, `vet`        |
| Pytest (Python) | `pytest`                      |
| C++ (CMake)     | `configure`, `build`          |
| C++ (GCC)       | `compile`                     |
| C++ (Clang)     | `compile`                     |
| C++ (MSVC)      | `compile`                     |

## Usage Examples

```bash
# Analyze Rust project
analyzer cargo check
analyzer cargo clippy
analyzer cargo test

# Analyze Python/Mypy project
analyzer mypy check
analyzer mypy check-strict

# Analyze Node.js project
analyzer npm lint
analyzer npm type-check
analyzer npm audit

# Analyze Java/Maven project
analyzer maven compile
analyzer maven test

# Analyze Java/Gradle project
analyzer gradle compile
analyzer gradle test

# Analyze Go project
analyzer go build
analyzer go test
analyzer go vet

# Analyze Python/Pytest
analyzer pytest

# Analyze C++ project with CMake
analyzer cpp cmake configure
analyzer cpp cmake build

# Analyze C++ project with GCC
analyzer cpp gcc compile

# Analyze C++ project with Clang
analyzer cpp clang compile

# Analyze C++ project with MSVC
analyzer cpp msvc compile
```

## Options

| Option                   | Description                                            |
| ------------------------ | ------------------------------------------------------ |
| `--filter-warnings`      | Filter out all warnings, only show errors              |
| `--filter-paths <paths>` | Filter errors by file paths (comma-separated)          |
| `--verbose`              | Show all issues without truncation                     |
| `--output <file>`        | Specify output file path (default: analysis_report.md) |

## Recommended Usage

### Quick Overview

Use without `--verbose` to get a quick overview of the project status:

```bash
analyzer cargo check
```

This shows top 20 files with most issues and up to 10 issues per file.

### Deep Dive into Specific Areas

Use `--verbose` with `--filter-paths` to fully analyze specific directories or files:

```bash
# Analyze all issues in a specific directory
analyzer cargo check --verbose --filter-paths src/core

# Analyze multiple specific directories
analyzer cargo check --verbose --filter-paths src/core,src/utils

# Analyze a specific file
analyzer cargo check --verbose --filter-paths src/main.rs
```

### CI/CD Integration

```bash
# Generate JSON report for CI/CD pipelines
analyzer cargo check --output report.json

# Check only errors, filter out warnings
analyzer cargo check --filter-warnings
```

## Configuration

Create `.analyzer.toml` in your project root:

```toml
version = "1.0"

[global]
default_format = "markdown"
filter_warnings = false

[commands.type-check]
exec = "npm run typecheck"
description = "Run TypeScript type checker"
tech_stacks = ["npm", "pnpm", "yarn"]

[tech_stack.npm]
test_framework = "jest"
```

## Report Formats

The tool generates reports in multiple formats:

- **Markdown**: Human-readable reports with statistics
- **JSON**: Machine-readable format for CI/CD
- **HTML**: Styled HTML reports

Reports include:

- Summary statistics
- Error and warning breakdown
- Top files with issues
- Detailed categorization with line numbers
