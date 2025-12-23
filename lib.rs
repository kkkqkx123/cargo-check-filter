//! Cargo错误分析库
//! 提供错误解析和统计功能

use std::collections::{HashMap, HashSet};

/// 错误信息结构
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub file_path: String,
    pub line_number: String,
    pub column_number: String,
    pub error_type: String,
    pub description: String,
}

/// 错误分类统计
#[derive(Debug, Default)]
pub struct ErrorStats {
    pub total_errors: usize,
    pub total_warnings: usize,
    pub unique_error_patterns: usize,
    pub unique_warning_patterns: usize,
    pub files_with_issues: HashSet<String>,
    pub error_type_distribution: HashMap<String, usize>,
    pub warning_type_distribution: HashMap<String, usize>,
    pub file_error_counts: HashMap<String, usize>,
    pub file_warning_counts: HashMap<String, usize>,
    pub categorized_errors: HashMap<String, HashMap<String, Vec<(String, String, String)>>>,
    pub categorized_warnings: HashMap<String, HashMap<String, Vec<(String, String, String)>>>,
}

/// 解析单行错误信息
pub fn parse_error_line(line: &str) -> Option<ErrorInfo> {
    let parts: Vec<&str> = line.splitn(5, ':').collect();
    if parts.len() < 5 {
        return None;
    }

    let file_path = parts[0].trim().to_string();
    let line_number = parts[1].trim().to_string();
    let column_number = parts[2].trim().to_string();
    let error_type = parts[3].trim().to_string();
    let description = parts[4].trim().to_string();

    if file_path.is_empty() || line_number.is_empty() || error_type.is_empty() {
        return None;
    }

    Some(ErrorInfo {
        file_path,
        line_number,
        column_number,
        error_type,
        description,
    })
}

/// 分类错误信息
pub fn categorize_errors(errors: &[ErrorInfo]) -> ErrorStats {
    let mut stats = ErrorStats::default();
    let mut error_patterns = HashSet::new();
    let mut warning_patterns = HashSet::new();

    for error in errors {
        stats.files_with_issues.insert(error.file_path.clone());
        
        let pattern_key = format!("{}: {}", error.error_type, error.description);
        
        if error.error_type.starts_with("error") {
            stats.total_errors += 1;
            error_patterns.insert(pattern_key.clone());
            
            *stats.error_type_distribution.entry(error.error_type.clone()).or_insert(0) += 1;
            *stats.file_error_counts.entry(error.file_path.clone()).or_insert(0) += 1;
            
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
            
            *stats.warning_type_distribution.entry(error.error_type.clone()).or_insert(0) += 1;
            *stats.file_warning_counts.entry(error.file_path.clone()).or_insert(0) += 1;
            
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_error_line() {
        let line = "src/main.rs:10:5: error[E0308]: mismatched types";
        let error = parse_error_line(line).unwrap();
        
        assert_eq!(error.file_path, "src/main.rs");
        assert_eq!(error.line_number, "10");
        assert_eq!(error.column_number, "5");
        assert_eq!(error.error_type, "error[E0308]");
        assert_eq!(error.description, "mismatched types");
    }
    
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