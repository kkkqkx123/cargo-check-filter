---
name: analyzer-usage
description: "Analyzes build tool errors from Cargo, NPM, PNPM, Yarn, Mypy, Pytest, Maven, Gradle, Go, and C++ (CMake/GCC/Clang/MSVC). Invoke when user asks to analyze build errors, check code quality issues, or use the analyzer CLI tool."
---

# Analyzer - Multi-language Build Tool Error Analyzer

This skill provides guidance on using the analyzer binary to analyze errors from various build tools and generate reports.

## Quick Start

```bash
analyzer <tech-stack> <subcommand> [options]
```

## Supported Tech Stacks

| Tech Stack      | Commands                                |
| --------------- | --------------------------------------- |
| Cargo (Rust)    | `check`, `clippy`, `clippy-all`, `test` |
| Mypy (Python)   | `check`, `check-strict`                 |
| Pytest (Python) | `test`, `test-quiet`, `test-verbose`    |
| NPM (Node.js)   | `lint`, `type-check`, `audit`           |
| PNPM (Node.js)  | `lint`, `type-check`, `audit`           |
| Yarn (Node.js)  | `lint`, `type-check`, `audit`           |
| Maven (Java)    | `compile`, `test`                       |
| Gradle (Java)   | `compile`, `test`                       |
| Go              | `build`, `vet`, `lint`                  |
| C++ (CMake)     | `check`, `build`                        |
| C++ (GCC)       | `check`                                 |
| C++ (Clang)     | `check`                                 |
| C++ (MSVC)      | `check`                                 |

## Common Usage Examples

```bash
# Rust/Cargo
analyzer cargo check
analyzer cargo clippy
analyzer cargo test

# Python
analyzer mypy check
analyzer pytest

# Node.js
analyzer npm lint
analyzer pnpm type-check

# Java
analyzer maven compile
analyzer gradle test

# Go
analyzer go build
analyzer go vet

# C++
analyzer cpp cmake build
analyzer cpp gcc check
```

## Global Options

| Option                   | Description                                            |
| ------------------------ | ------------------------------------------------------ |
| `-h, --help`             | Show help message                                      |
| `-v, --version`          | Show version                                           |
| `--filter-warnings`      | Filter out all warnings, only show errors              |
| `--filter-paths <paths>` | Filter errors by file paths (comma-separated)          |
| `--verbose`              | Show all issues without truncation                     |
| `-o, --output <file>`    | Specify output file path (default: analysis_report.md) |

## References

- [Cargo Workspace & Target Options](references/cargo-options.md) - Detailed Cargo-specific options
- [Configuration Guide](references/configuration-guide.md) - How to configure the analyzer
- [Report Formats](references/report-formats.md) - Available output formats and their structure
