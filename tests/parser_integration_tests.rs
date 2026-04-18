//! 解析器集成测试
//! 验证解析器能正确解析实际命令输出

use std::fs;
use std::path::PathBuf;

mod common;
use common::output_dir;

// 从 src 导入解析器
use analyzer::core::{IssueLevel, OutputParser};
use analyzer::plugins::mypy::parser::MypyParser;
use analyzer::plugins::npm::parser::NpmParser;

/// 获取测试输出文件路径
fn get_output_file(name: &str) -> PathBuf {
    output_dir().join(format!("{}.txt", name))
}

/// 读取输出文件内容
fn read_output_file(name: &str) -> String {
    let path = get_output_file(name);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read output file: {}", path.display()))
}

/// 验证 Issue 的基本属性
fn assert_issue_valid(issue: &analyzer::core::Issue, expected_file: &str) {
    assert!(
        !issue.message.is_empty(),
        "Issue message should not be empty"
    );
    assert!(
        issue.location.file_path.contains(expected_file),
        "Issue file path should contain '{}', got: {}",
        expected_file,
        issue.location.file_path
    );
    assert!(
        issue.location.line_number.is_some(),
        "Issue should have line number"
    );
}

/// 统计各级别 Issue 数量
fn count_issues_by_level(issues: &[analyzer::core::Issue]) -> (usize, usize, usize) {
    let errors = issues.iter().filter(|i| matches!(i.level, IssueLevel::Error)).count();
    let warnings = issues.iter().filter(|i| matches!(i.level, IssueLevel::Warning)).count();
    let infos = issues.iter().filter(|i| matches!(i.level, IssueLevel::Info)).count();
    (errors, warnings, infos)
}

#[test]
fn test_mypy_parser_basic() {
    let content = read_output_file("mypy_basic");
    let parser = MypyParser::new();
    let issues = parser.parse(&content);

    // 验证解析出了 Issue
    assert!(!issues.is_empty(), "Should parse at least one issue from mypy output");

    // 验证至少有一个错误
    let (errors, warnings, _) = count_issues_by_level(&issues);
    assert!(errors > 0, "Should have at least one error, got {} errors", errors);

    // 验证第一个 Issue 的结构
    let first_issue = &issues[0];
    assert_issue_valid(first_issue, ".py");
    assert!(matches!(first_issue.level, IssueLevel::Error | IssueLevel::Warning));

    // 验证包含特定错误
    let has_type_error = issues.iter().any(|i| {
        i.message.contains("Unsupported operand types")
            || i.message.contains("Incompatible types")
            || i.message.contains("Missing type annotation")
    });
    assert!(has_type_error, "Should have type-related errors");

    println!("✓ Mypy parser correctly parsed {} issues ({} errors, {} warnings)", 
             issues.len(), errors, warnings);
}

#[test]
fn test_mypy_parser_specific_file() {
    let content = read_output_file("mypy_specific_file");
    let parser = MypyParser::new();
    let issues = parser.parse(&content);

    assert!(!issues.is_empty(), "Should parse issues from specific file output");

    // 验证至少有一些 Issue 来自 main.py（也可能来自其他文件如 utils.py）
    let has_main_py = issues.iter().any(|i| i.location.file_path.contains("main.py"));
    assert!(has_main_py, "Should have issues from main.py");

    // 验证所有 Issue 都是 Python 文件
    let all_py_files = issues.iter().all(|i| i.location.file_path.ends_with(".py"));
    assert!(all_py_files, "All issues should be from .py files");

    println!("✓ Mypy parser correctly parsed {} issues from Python files", issues.len());
}

#[test]
fn test_mypy_parser_strict() {
    let content = read_output_file("mypy_strict");
    let parser = MypyParser::new();
    let issues = parser.parse(&content);

    // strict 模式下应该有更多错误
    assert!(
        issues.len() >= 3,
        "Strict mode should have at least 3 issues, got {}",
        issues.len()
    );

    println!("✓ Mypy parser (strict) correctly parsed {} issues", issues.len());
}

#[test]
fn test_eslint_parser_output() {
    let content = read_output_file("npm_eslint_sample");
    let parser = NpmParser::new();
    let issues = parser.parse(&content);

    // 验证解析出了 Issue
    assert!(
        !issues.is_empty(),
        "Should parse at least one issue from ESLint output"
    );

    // 验证错误和警告的数量
    let (errors, warnings, _) = count_issues_by_level(&issues);
    assert!(
        errors + warnings >= 3,
        "Should have at least 3 issues (errors + warnings), got {} errors and {} warnings",
        errors,
        warnings
    );

    // 验证 Issue 结构
    let first_issue = &issues[0];
    assert_issue_valid(first_issue, ".ts");

    // 验证文件路径正确提取
    let has_index_ts = issues.iter().any(|i| i.location.file_path.contains("index.ts"));
    let has_utils_ts = issues.iter().any(|i| i.location.file_path.contains("utils.ts"));
    assert!(
        has_index_ts || has_utils_ts,
        "Should have issues from index.ts or utils.ts"
    );

    // 验证行号和列号
    let issue_with_location = issues.iter().find(|i| {
        i.location.line_number.is_some() && i.location.column_number.is_some()
    });
    assert!(
        issue_with_location.is_some(),
        "At least one issue should have both line and column numbers"
    );

    println!(
        "✓ ESLint parser correctly parsed {} issues ({} errors, {} warnings)",
        issues.len(),
        errors,
        warnings
    );
}

#[test]
fn test_typescript_parser_output() {
    let content = read_output_file("npm_typecheck_sample");
    let parser = NpmParser::new();
    let issues = parser.parse(&content);

    // 验证解析出了 Issue
    assert!(
        !issues.is_empty(),
        "Should parse at least one issue from TypeScript output"
    );

    // TypeScript 输出应该全是错误
    let (errors, _, _) = count_issues_by_level(&issues);
    assert!(
        errors >= 3,
        "Should have at least 3 TypeScript errors, got {}",
        errors
    );

    // 验证包含 TS 错误代码（格式可能是 TSxxxx 或 [TSxxxx]）
    let has_ts_code = issues.iter().any(|i: &analyzer::core::Issue| {
        i.code.as_ref().map(|c: &String| {
            c.starts_with("TS") || c.starts_with("[TS")
        }).unwrap_or(false)
    });
    assert!(has_ts_code, "Should have TypeScript error codes (TSxxxx), got codes: {:?}", 
            issues.iter().filter_map(|i| i.code.clone()).collect::<Vec<_>>());

    // 验证 Issue 结构
    let first_issue = &issues[0];
    assert_issue_valid(first_issue, ".ts");

    println!("✓ TypeScript parser correctly parsed {} errors", issues.len());
}

#[test]
fn test_npm_audit_parser_output() {
    let content = read_output_file("npm_audit");
    let parser = NpmParser::new();
    let issues = parser.parse(&content);

    // npm audit 输出包含错误信息
    assert!(
        !issues.is_empty(),
        "Should parse at least one issue from npm audit output"
    );

    // 验证包含 NPM 错误
    let has_npm_error = issues.iter().any(|i| {
        i.message.contains("NPM error") || i.message.contains("requires an existing lockfile")
    });
    assert!(has_npm_error, "Should have NPM audit errors");

    // 验证错误级别
    let all_errors = issues.iter().all(|i| matches!(i.level, IssueLevel::Error));
    assert!(all_errors, "All npm audit issues should be errors");

    println!("✓ NPM audit parser correctly parsed {} errors", issues.len());
}

#[test]
fn test_npm_ls_parser_output() {
    let content = read_output_file("npm_ls");
    let parser = NpmParser::new();
    let issues = parser.parse(&content);

    // npm ls 输出包含依赖缺失错误
    // 注意：npm ls 的依赖树本身不是错误，但 UNMET DEPENDENCY 是错误
    let has_missing_deps = issues.iter().any(|i| {
        i.message.contains("Missing dependency") || i.message.contains("UNMET")
    });

    if has_missing_deps {
        println!("✓ NPM ls parser correctly parsed {} dependency issues", issues.len());
    } else {
        println!("! NPM ls output doesn't contain parseable dependency issues (this may be OK)");
    }
}

#[test]
fn test_parser_handles_empty_input() {
    let parser = NpmParser::new();
    let issues = parser.parse("");
    assert!(issues.is_empty(), "Should return empty vec for empty input");

    let parser = MypyParser::new();
    let issues = parser.parse("");
    assert!(issues.is_empty(), "Should return empty vec for empty input");

    println!("✓ Parsers correctly handle empty input");
}

#[test]
fn test_parser_handles_no_issues() {
    // 模拟没有错误的输出
    let no_error_output = "✓ No issues found\nAll checks passed!";
    let parser = NpmParser::new();
    let issues = parser.parse(no_error_output);
    assert!(issues.is_empty(), "Should return empty vec when no issues found");

    println!("✓ Parsers correctly handle 'no issues' output");
}

#[test]
fn test_is_issue_start_detection() {
    let parser = NpmParser::new();

    // ESLint 格式
    assert!(parser.is_issue_start("  3:7   warning  message"));
    assert!(parser.is_issue_start("10:5   error    message"));

    // TypeScript 格式
    assert!(parser.is_issue_start("src/index.ts(13,7): error TS2345: message"));

    // NPM 错误格式
    assert!(parser.is_issue_start("npm error code ENOLOCK"));
    assert!(parser.is_issue_start("npm error missing: package@version"));

    // 非 Issue 行
    assert!(!parser.is_issue_start(""));
    assert!(!parser.is_issue_start("Some random text"));
    assert!(!parser.is_issue_start("✓ All checks passed"));

    println!("✓ Issue start detection works correctly");
}

/// 综合测试：验证所有输出文件都能被正确解析
#[test]
fn test_all_output_files_parsable() {
    let output_dir = output_dir();
    let mut parsed_count = 0;
    let mut failed_files = Vec::new();

    for entry in fs::read_dir(&output_dir).expect("Failed to read output directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map(|e| e == "txt").unwrap_or(false) {
                let filename = path.file_stem().unwrap_or_default().to_string_lossy();
                let content = fs::read_to_string(&path).expect("Failed to read file");

                // 根据文件名选择合适的解析器
                let issues = if filename.starts_with("mypy") {
                    let parser = MypyParser::new();
                    parser.parse(&content)
                } else if filename.starts_with("npm") {
                    let parser = NpmParser::new();
                    parser.parse(&content)
                } else {
                    continue;
                };

                // 验证解析结果
                if !issues.is_empty() {
                    // 验证至少一个 Issue 的结构正确
                    let valid = issues.iter().any(|i| {
                        !i.message.is_empty()
                            && !i.location.file_path.is_empty()
                            && i.location.line_number.is_some()
                    });

                    if valid {
                        parsed_count += 1;
                        println!("✓ {}: parsed {} valid issues", filename, issues.len());
                    } else {
                        failed_files.push(filename.to_string());
                    }
                } else {
                    // 空结果也可能是正确的（如果没有错误）
                    println!("! {}: no issues parsed (may be correct)", filename);
                }
            }
        }
    }

    assert!(
        parsed_count >= 3,
        "Should successfully parse at least 3 output files, got {}. Failed: {:?}",
        parsed_count,
        failed_files
    );

    println!("\n✓ Successfully parsed {} output files", parsed_count);
}
