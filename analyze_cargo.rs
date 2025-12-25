//! Cargo错误分析工具 - Rust版本
//! 分析cargo check输出，生成详细的错误报告

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{Write};
use std::process::Command;

/// 错误信息结构
#[derive(Debug, Clone)]
struct ErrorInfo {
    file_path: String,
    line_number: String,
    column_number: String,
    error_type: String,
    description: String,
}

/// 错误分类统计
#[derive(Debug, Default)]
struct ErrorStats {
    total_errors: usize,
    total_warnings: usize,
    unique_error_patterns: usize,
    unique_warning_patterns: usize,
    files_with_issues: HashSet<String>,
    error_type_distribution: HashMap<String, usize>,
    warning_type_distribution: HashMap<String, usize>,
    file_error_counts: HashMap<String, usize>,
    file_warning_counts: HashMap<String, usize>,
    categorized_errors: HashMap<String, HashMap<String, Vec<(String, String, String)>>>,
    categorized_warnings: HashMap<String, HashMap<String, Vec<(String, String, String)>>>,
}

/// 解析错误信息行（处理多行格式）
fn parse_error_line(lines: &[String], start_index: usize) -> (Option<ErrorInfo>, usize) {
    if start_index >= lines.len() {
        return (None, start_index);
    }

    let line = &lines[start_index];

    // Check if this line starts with error/warning level
    if line.starts_with("error:") || line.starts_with("warning:") {
        let colon_pos = line.find(':');
        if let Some(colon_pos) = colon_pos {
            let error_type = line[..colon_pos].trim().to_string();
            let error_desc = line[colon_pos + 1..].trim().to_string();

            // Look for the next line with arrow format pointing to file location
            if start_index + 1 < lines.len() {
                let next_line = &lines[start_index + 1];
                let trimmed_next = next_line.trim();

                // Check if the line starts with arrow " --> "
                if trimmed_next.starts_with("-->") {
                    // Extract file:line:col after the arrow
                    let after_arrow = &trimmed_next[3..].trim(); // Remove " --> " prefix

                    // Split by colons to get file:line:col
                    let parts: Vec<&str> = after_arrow.rsplitn(3, ':').collect();
                    if parts.len() == 3 {
                        let _col = parts[0];
                        let _line_num = parts[1];
                        let _file_path = parts[2];

                        // The file path might still have a colon in it (e.g., C:\path\to\file.rs)
                        // So we need to reassemble it properly
                        if after_arrow.contains(':') && !after_arrow.starts_with(':') {
                            // Find the last two colons for line and column, everything before that is the file path
                            let mut colon_positions = Vec::new();
                            for (i, c) in after_arrow.char_indices() {
                                if c == ':' {
                                    colon_positions.push(i);
                                }
                            }

                            if colon_positions.len() >= 2 {
                                let last_colon = colon_positions[colon_positions.len() - 1];
                                let second_last_colon = colon_positions[colon_positions.len() - 2];

                                let col_part = &after_arrow[last_colon + 1..];
                                let line_part = &after_arrow[second_last_colon + 1..last_colon];
                                let file_part = &after_arrow[..second_last_colon];

                                if col_part.chars().all(|c| c.is_ascii_digit()) &&
                                   line_part.chars().all(|c| c.is_ascii_digit()) {
                                    let file_path = file_part.trim().to_string();
                                    let line_number = line_part.to_string();
                                    let column_number = col_part.to_string();

                                    if !file_path.is_empty() && !error_type.is_empty() && !error_desc.is_empty() {
                                        return (
                                            Some(ErrorInfo {
                                                file_path,
                                                line_number,
                                                column_number,
                                                error_type,
                                                description: error_desc,
                                            }),
                                            start_index + 2 // Skip both the error line and the arrow line
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Try old format if the multi-line format doesn't match
    let parts: Vec<&str> = line.splitn(5, ':').collect();
    if parts.len() >= 5 {
        let file_path = parts[0].trim().to_string();
        let line_number = parts[1].trim().to_string();
        let column_number = parts[2].trim().to_string();
        let error_type = parts[3].trim().to_string();
        let description = parts[4].trim().to_string();

        if !file_path.is_empty() && !line_number.is_empty() && !error_type.is_empty() {
            return (Some(ErrorInfo {
                file_path,
                line_number,
                column_number,
                error_type,
                description,
            }), start_index + 1);
        }
    }

    (None, start_index + 1)
}

/// 运行cargo test --lib并捕获输出
fn run_cargo_test() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(["test", "--lib", "--message-format=short"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);
    let lines: Vec<String> = combined.lines().map(|s| s.to_string()).collect();

    Ok(lines)
}

/// 分类错误信息
fn categorize_errors(errors: &[ErrorInfo]) -> ErrorStats {
    let mut stats = ErrorStats::default();
    let mut error_patterns = HashSet::new();
    let mut warning_patterns = HashSet::new();

    for error in errors {
        stats.files_with_issues.insert(error.file_path.clone());
        
        let pattern_key = format!("{}: {}", error.error_type, error.description);
        
        if error.error_type.starts_with("error") {
            stats.total_errors += 1;
            error_patterns.insert(pattern_key.clone());
            
            // 错误类型分布
            *stats.error_type_distribution.entry(error.error_type.clone()).or_insert(0) += 1;
            
            // 文件错误计数
            *stats.file_error_counts.entry(error.file_path.clone()).or_insert(0) += 1;
            
            // 分类错误
            let file_errors = stats.categorized_errors
                .entry(error.error_type.clone())
                .or_insert_with(HashMap::new);
            
            let error_list = file_errors
                .entry(error.file_path.clone())
                .or_insert_with(Vec::new);
            
            error_list.push((error.line_number.clone(), error.column_number.clone(), error.description.clone()));
        } else if error.error_type.starts_with("warning") {
            stats.total_warnings += 1;
            warning_patterns.insert(pattern_key.clone());
            
            // 警告类型分布
            *stats.warning_type_distribution.entry(error.error_type.clone()).or_insert(0) += 1;
            
            // 文件警告计数
            *stats.file_warning_counts.entry(error.file_path.clone()).or_insert(0) += 1;
            
            // 分类警告
            let file_warnings = stats.categorized_warnings
                .entry(error.error_type.clone())
                .or_insert_with(HashMap::new);
            
            let warning_list = file_warnings
                .entry(error.file_path.clone())
                .or_insert_with(Vec::new);
            
            warning_list.push((error.line_number.clone(), error.column_number.clone(), error.description.clone()));
        }
    }

    stats.unique_error_patterns = error_patterns.len();
    stats.unique_warning_patterns = warning_patterns.len();

    stats
}

/// 生成Markdown报告
fn generate_markdown_report(stats: &ErrorStats, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(output_path)?;
    
    // 报告标题
    writeln!(file, "# Cargo Check Error Analysis Report\n")?;
    
    // 摘要部分
    writeln!(file, "## Summary\n")?;
    writeln!(file, "- **Total Errors**: {}", stats.total_errors)?;
    writeln!(file, "- **Total Warnings**: {}", stats.total_warnings)?;
    writeln!(file, "- **Total Issues**: {}", stats.total_errors + stats.total_warnings)?;
    writeln!(file, "- **Unique Error Patterns**: {}", stats.unique_error_patterns)?;
    writeln!(file, "- **Unique Warning Patterns**: {}", stats.unique_warning_patterns)?;
    writeln!(file, "- **Files with Issues**: {}\n", stats.files_with_issues.len())?;
    
    // 错误统计
    writeln!(file, "## Error Statistics\n")?;
    writeln!(file, "**Total Errors**: {}\n", stats.total_errors)?;
    
    // 错误类型分布
    if !stats.error_type_distribution.is_empty() {
        writeln!(file, "### Error Type Breakdown\n")?;
        let mut error_types: Vec<(&String, &usize)> = stats.error_type_distribution.iter().collect();
        error_types.sort_by(|a, b| b.1.cmp(a.1));
        
        for (error_type, count) in error_types {
            writeln!(file, "- **{}**: {} errors", error_type, count)?;
        }
        writeln!(file)?;
    }
    
    // 文件错误排名
    if !stats.file_error_counts.is_empty() {
        writeln!(file, "### Files with Errors (Top 10)\n")?;
        let mut file_counts: Vec<(&String, &usize)> = stats.file_error_counts.iter().collect();
        file_counts.sort_by(|a, b| b.1.cmp(a.1));
        
        for (file_path, count) in file_counts.iter().take(10) {
            writeln!(file, "- `{}`: {} errors", file_path, count)?;
        }
        writeln!(file)?;
    }
    
    // 警告统计
    writeln!(file, "## Warning Statistics\n")?;
    writeln!(file, "**Total Warnings**: {}\n", stats.total_warnings)?;
    
    // 警告类型分布
    if !stats.warning_type_distribution.is_empty() {
        writeln!(file, "### Warning Type Breakdown\n")?;
        let mut warning_types: Vec<(&String, &usize)> = stats.warning_type_distribution.iter().collect();
        warning_types.sort_by(|a, b| b.1.cmp(a.1));
        
        for (warning_type, count) in warning_types {
            writeln!(file, "- **{}**: {} warnings", warning_type, count)?;
        }
        writeln!(file)?;
    }
    
    // 文件警告排名
    if !stats.file_warning_counts.is_empty() {
        writeln!(file, "### Files with Warnings (Top 10)\n")?;
        let mut file_counts: Vec<(&String, &usize)> = stats.file_warning_counts.iter().collect();
        file_counts.sort_by(|a, b| b.1.cmp(a.1));
        
        for (file_path, count) in file_counts.iter().take(10) {
            writeln!(file, "- `{}`: {} warnings", file_path, count)?;
        }
        writeln!(file)?;
    }
    
    // 详细错误分类
    if !stats.categorized_errors.is_empty() {
        writeln!(file, "## Detailed Error Categorization\n")?;
        
        let mut error_categories: Vec<(&String, &HashMap<String, Vec<(String, String, String)>>)> = 
            stats.categorized_errors.iter().collect();
        error_categories.sort_by(|a, b| {
            let count_a: usize = b.1.values().map(|v| v.len()).sum();
            let count_b: usize = a.1.values().map(|v| v.len()).sum();
            count_a.cmp(&count_b)
        });
        
        for (error_type, files) in error_categories {
            let total_occurrences: usize = files.values().map(|v| v.len()).sum();
            let unique_files = files.len();
            
            writeln!(file, "### {}: {}\n", error_type, files.values().next().unwrap()[0].2)?;
            writeln!(file, "**Total Occurrences**: {}  ", total_occurrences)?;
            writeln!(file, "**Unique Files**: {}\n", unique_files)?;
            
            let mut file_entries: Vec<(&String, &Vec<(String, String, String)>)> = files.iter().collect();
            file_entries.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            
            for (file_path, errors) in file_entries {
                writeln!(file, "#### `{}`: {} occurrences\n", file_path, errors.len())?;
                
                for (i, (line_num, _col_num, desc)) in errors.iter().enumerate() {
                    if i < 3 { // 只显示前3个示例
                        writeln!(file, "- Line {}: {}", line_num, desc)?;
                    }
                }
                
                if errors.len() > 3 {
                    writeln!(file, "- ... {} more occurrences in this file", errors.len() - 3)?;
                }
                writeln!(file)?;
            }
        }
    }
    
    // 详细警告分类
    if !stats.categorized_warnings.is_empty() {
        writeln!(file, "## Detailed Warning Categorization\n")?;
        
        let mut warning_categories: Vec<(&String, &HashMap<String, Vec<(String, String, String)>>)> = 
            stats.categorized_warnings.iter().collect();
        warning_categories.sort_by(|a, b| {
            let count_a: usize = b.1.values().map(|v| v.len()).sum();
            let count_b: usize = a.1.values().map(|v| v.len()).sum();
            count_a.cmp(&count_b)
        });
        
        for (warning_type, files) in warning_categories {
            let total_occurrences: usize = files.values().map(|v| v.len()).sum();
            let unique_files = files.len();
            
            writeln!(file, "### {}: {}\n", warning_type, files.values().next().unwrap()[0].2)?;
            writeln!(file, "**Total Occurrences**: {}  ", total_occurrences)?;
            writeln!(file, "**Unique Files**: {}\n", unique_files)?;
            
            let mut file_entries: Vec<(&String, &Vec<(String, String, String)>)> = files.iter().collect();
            file_entries.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            
            for (file_path, warnings) in file_entries {
                writeln!(file, "#### `{}`: {} occurrences\n", file_path, warnings.len())?;
                
                for (i, (line_num, _col_num, desc)) in warnings.iter().enumerate() {
                    if i < 3 { // 只显示前3个示例
                        writeln!(file, "- Line {}: {}", line_num, desc)?;
                    }
                }
                
                if warnings.len() > 3 {
                    writeln!(file, "- ... {} more occurrences in this file", warnings.len() - 3)?;
                }
                writeln!(file)?;
            }
        }
    }
    
    Ok(())
}

/// 命令行参数结构体
#[derive(Debug)]
struct Args {
    output: String,
    filter_warnings: bool,
    filter_paths: Vec<String>,
}

/// 解析命令行参数
fn parse_arguments() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut output_file = "cargo_errors_report.md".to_string();
    let mut filter_warnings = false;
    let mut filter_paths = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 1;
                }
            }
            "--filter-warnings" => {
                filter_warnings = true;
            }
            "--filter-paths" => {
                if i + 1 < args.len() {
                    let paths: Vec<String> = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    filter_paths.extend(paths);
                    i += 1;
                }
            }
            "--help" => {
                show_help();
                std::process::exit(0);
            }
            _ => {
                if args[i].starts_with("-") {
                    eprintln!("未知选项: {}", args[i]);
                    show_help();
                    std::process::exit(1);
                }
            }
        }
        i += 1;
    }

    Args {
        output: output_file,
        filter_warnings,
        filter_paths,
    }
}

/// 显示帮助信息
fn show_help() {
    println!("用法: analyze_cargo_errors [选项]");
    println!();
    println!("选项:");
    println!("  --help                 显示此帮助信息");
    println!("  --output <文件>        指定输出文件路径（默认: cargo_errors_report.md）");
    println!("  --filter-warnings      过滤警告，仅显示错误");
    println!("  --filter-paths <路径>  按文件路径过滤错误（逗号分隔）");
    println!();
    println!("示例:");
    println!("  analyze_cargo_errors");
    println!("  analyze_cargo_errors --output report.md");
    println!("  analyze_cargo_errors --filter-warnings");
    println!("  analyze_cargo_errors --filter-paths src/main.rs,src/lib.rs");
    println!("  analyze_cargo_errors --filter-warnings --output errors_only.md");
}

/// 检查是否应该包含错误
fn should_include_error(file_path: &str, error_type: &str, args: &Args, base_dir: &str) -> bool {
    // 过滤警告
    if args.filter_warnings && error_type.starts_with("warning") {
        return false;
    }
    
    // 过滤路径
    if !args.filter_paths.is_empty() {
        let relative_path = file_path.replace(base_dir, "").trim_start_matches('\\').to_string();
        let include = args.filter_paths.iter().any(|filter_path| {
            relative_path.starts_with(filter_path) || file_path.contains(filter_path)
        });
        
        if !include {
            return false;
        }
    }
    
    true
}

/// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_arguments();
    let base_dir = std::env::current_dir()?.to_string_lossy().to_string();
    
    println!("正在运行cargo test --lib...");

    // 运行cargo test
    let lines = run_cargo_test()?;
    
    // 解析错误信息
    let mut all_errors = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let (error_info, new_index) = parse_error_line(&lines, i);
        if let Some(error_info) = error_info {
            all_errors.push(error_info);
        }
        i = new_index;
    }
    
    println!("找到 {} 个错误/警告信息", all_errors.len());
    
    // 应用过滤器
    let mut filtered_errors = Vec::new();
    for error in all_errors {
        if should_include_error(&error.file_path, &error.error_type, &args, &base_dir) {
            filtered_errors.push(error);
        }
    }
    
    if filtered_errors.is_empty() {
        println!("应用过滤器后，没有错误/警告信息");
        return Ok(());
    }
    
    println!("应用过滤器后，剩余 {} 个错误/警告信息", filtered_errors.len());
    
    // 分类错误
    let stats = categorize_errors(&filtered_errors);
    
    // 生成报告
    println!("生成Markdown报告...");
    generate_markdown_report(&stats, &args.output)?;
    
    println!("报告已生成: {}", args.output);
    println!();
    println!("统计摘要:");
    println!("  - 总错误数: {}", stats.total_errors);
    println!("  - 总警告数: {}", stats.total_warnings);
    println!("  - 总问题数: {}", stats.total_errors + stats.total_warnings);
    println!("  - 唯一错误模式: {}", stats.unique_error_patterns);
    println!("  - 唯一警告模式: {}", stats.unique_warning_patterns);
    println!("  - 涉及文件数: {}", stats.files_with_issues.len());
    
    // 显示过滤器摘要
    if args.filter_warnings {
        println!("过滤器: 已过滤警告，仅显示错误");
    }
    if !args.filter_paths.is_empty() {
        println!("过滤器: 仅显示来自路径的错误: {}", args.filter_paths.join(", "));
    }
    
    // 显示前5个错误模式
    if !stats.categorized_errors.is_empty() {
        println!("\n前5个错误模式:");
        let mut error_categories: Vec<(&String, &HashMap<String, Vec<(String, String, String)>>)> = 
            stats.categorized_errors.iter().collect();
        error_categories.sort_by(|a, b| {
            let count_a: usize = b.1.values().map(|v| v.len()).sum();
            let count_b: usize = a.1.values().map(|v| v.len()).sum();
            count_a.cmp(&count_b)
        });
        
        for (error_type, files) in error_categories.iter().take(5) {
            let count: usize = files.values().map(|v| v.len()).sum();
            println!("- {}: {} 次出现", error_type, count);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_errors() {
        let errors = vec![
            ErrorInfo {
                file_path: "src/main.rs".to_string(),
                line_number: "10".to_string(),
                column_number: "5".to_string(),
                error_type: "error[E0308]".to_string(),
                description: "mismatched types".to_string(),
            },
            ErrorInfo {
                file_path: "src/lib.rs".to_string(),
                line_number: "15".to_string(),
                column_number: "3".to_string(),
                error_type: "warning".to_string(),
                description: "unused variable".to_string(),
            },
        ];

        let stats = categorize_errors(&errors);

        assert_eq!(stats.total_errors, 1);
        assert_eq!(stats.total_warnings, 1);
        assert_eq!(stats.files_with_issues.len(), 2);
    }
}