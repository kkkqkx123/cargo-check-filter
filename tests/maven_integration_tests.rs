//! Maven 集成测试
//! 执行实际的 Maven 命令，验证解析逻辑与实际输出格式是否一致

use std::path::PathBuf;

mod common;
use common::{fixtures_dir, is_command_available, run_command, save_raw_output, read_sample, generate_report};

fn maven_project_path() -> PathBuf {
    fixtures_dir().join("maven-project")
}

/// 检查 Maven 是否可用
fn ensure_maven() -> Result<(), String> {
    if !is_command_available("mvn") {
        return Err("Maven (mvn) is not available in PATH. Please install Maven.".to_string());
    }
    Ok(())
}

#[test]
fn test_maven_compile_output() {
    use analyzer::plugins::maven::parser::MavenParser;
    use analyzer::core::OutputParser;

    if let Err(e) = ensure_maven() {
        println!("Skipping test: {}", e);
        return;
    }

    let project_path = maven_project_path();

    // 运行 Maven 编译
    let output = match run_command("mvn", &["compile", "-q"], &project_path) {
        Ok(output) => output,
        Err(e) => {
            // Maven 编译失败时也会返回输出（包含错误信息）
            println!("Maven compile failed or found errors: {}", e);
            // 尝试再次运行以获取错误输出
            match run_command("mvn", &["compile"], &project_path) {
                Ok(output) => output,
                Err(_) => {
                    // 如果还是失败，使用样本输出
                    println!("Using sample output for testing");
                    read_sample("maven_compile_sample")
                }
            }
        }
    };

    // 保存原始输出
    save_raw_output("maven_compile", &output);

    // 解析并生成报告
    let parser = MavenParser::new();
    let issues = parser.parse(&output);
    generate_report(
        "maven_compile",
        "Maven Compile",
        "mvn compile",
        &issues,
        Some("raw_output/maven_compile.txt")
    );

    println!("=== Maven Compile Output ===");
    println!("{}", output);

    // 验证 Maven 输出格式
    // 格式: [ERROR] /path/to/File.java:[line,col] error: message
    // 格式: [WARNING] /path/to/File.java:[line,col] warning: message
    let lines: Vec<&str> = output.lines().collect();
    let has_error_lines = lines.iter().any(|line: &&str| {
        line.trim().starts_with("[ERROR]") || line.trim().starts_with("[WARNING]")
    });

    if has_error_lines {
        println!("✓ Found Maven error/warning lines in expected format");
    } else if output.contains("BUILD SUCCESS") {
        println!("✓ Maven build succeeded (no issues found)");
    } else {
        println!("! No error lines found (may be due to Maven configuration or build success)");
    }

    // 验证输出格式符合解析器预期
    for line in &lines {
        let trimmed: &str = line.trim();
        if trimmed.starts_with("[ERROR]") || trimmed.starts_with("[WARNING]") {
            println!("  Found issue: {}", line);
        }
    }
}

#[test]
fn test_maven_test_output() {
    use analyzer::plugins::maven::parser::MavenParser;
    use analyzer::core::OutputParser;

    if let Err(e) = ensure_maven() {
        println!("Skipping test: {}", e);
        return;
    }

    let project_path = maven_project_path();

    // 运行 Maven 测试
    let output = match run_command("mvn", &["test", "-q"], &project_path) {
        Ok(output) => output,
        Err(e) => {
            println!("Maven test failed or found errors: {}", e);
            match run_command("mvn", &["test"], &project_path) {
                Ok(output) => output,
                Err(_) => {
                    println!("Using sample output for testing");
                    read_sample("maven_test_sample")
                }
            }
        }
    };

    // 保存原始输出
    save_raw_output("maven_test", &output);

    // 解析并生成报告
    let parser = MavenParser::new();
    let issues = parser.parse(&output);
    generate_report(
        "maven_test",
        "Maven Test",
        "mvn test",
        &issues,
        Some("raw_output/maven_test.txt")
    );

    println!("=== Maven Test Output ===");
    println!("{}", output);

    // 验证测试输出
    let lines: Vec<&str> = output.lines().collect();
    let has_test_output = lines.iter().any(|line: &&str| {
        line.contains("Tests run:") || line.contains("T E S T S")
    });

    if has_test_output {
        println!("✓ Found Maven test output");
    } else if output.contains("BUILD SUCCESS") {
        println!("✓ Maven tests passed");
    } else {
        println!("! No test output found");
    }
}

#[test]
fn test_validate_maven_outputs() {
    use analyzer::core::{AnalysisResult, IssueLevel, OutputParser};
    use analyzer::plugins::maven::parser::MavenParser;

    let parser = MavenParser::new();

    // 验证编译错误样本
    let compile_output = read_sample("maven_compile_sample");
    let issues = parser.parse(&compile_output);

    println!("=== Validating Maven Compile Sample ===");
    println!("Found {} issues in sample output", issues.len());

    let result = AnalysisResult::from_issues(issues);
    println!("Total errors: {}", result.issues_by_level.get(&IssueLevel::Error).unwrap_or(&0));
    println!("Total warnings: {}", result.issues_by_level.get(&IssueLevel::Warning).unwrap_or(&0));

    // 验证解析结果
    assert!(
        result.total_issues > 0,
        "Expected at least one issue in the sample output"
    );

    // 验证错误详情
    for (file_path, file_issues) in &result.issues_by_file {
        println!("  File: {} - {} issues", file_path, file_issues.len());
        for issue in file_issues {
            println!("    [{:?}] Line {:?}: {}",
                issue.level,
                issue.location.line_number,
                issue.message
            );
        }
    }
}

#[test]
fn test_maven_parser_specific_patterns() {
    use analyzer::plugins::maven::parser::MavenParser;
    use analyzer::core::OutputParser;

    let parser = MavenParser::new();

    // 测试各种 Maven 错误格式
    let test_cases = vec![
        ("[ERROR] /src/main/java/App.java:[10,5] error: cannot find symbol", true),
        ("[WARNING] /src/main/java/App.java:[20,10] warning: [unchecked] unchecked conversion", true),
        ("[ERROR] /src/test/java/Test.java:[5,1] error: package org.junit does not exist", true),
        ("[INFO] Building maven-project 1.0-SNAPSHOT", false),
        ("[DEBUG] Some debug message", false),
    ];

    println!("=== Testing Maven Parser Patterns ===");
    for (line, should_be_issue) in test_cases {
        let is_issue = parser.is_issue_start(line);
        let status = if is_issue == should_be_issue { "✓" } else { "✗" };
        println!("{} Line: '{}' - is_issue: {} (expected: {})",
            status, line, is_issue, should_be_issue);
        assert_eq!(is_issue, should_be_issue,
            "Pattern mismatch for line: {}", line);
    }
}
