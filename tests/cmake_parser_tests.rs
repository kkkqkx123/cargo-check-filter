//! CMake Parser Unit Tests
//! Test CMakeParser for CMake configuration and build output

use std::fs;

mod common;
use common::samples_dir;

use analyzer::core::{IssueLevel, OutputParser};
use analyzer::plugins::cpp::cmake::parser::CMakeParser;

/// Read sample file content
fn read_sample(name: &str) -> String {
    let path = samples_dir().join(format!("{}.txt", name));
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read sample file {}: {}", path.display(), e))
}

#[test]
fn test_cmake_parser_basic() {
    let content = read_sample("cmake_basic_sample");
    let parser = CMakeParser::new();
    let issues = parser.parse(&content);

    // Should parse both CMake errors/warnings and compiler errors
    assert!(!issues.is_empty(), "Should parse issues from CMake output");

    // Check for CMake errors
    let cmake_errors: Vec<_> = issues.iter()
        .filter(|i| i.code.as_ref().map_or(false, |c| c.contains("CMake Error")))
        .collect();
    assert!(!cmake_errors.is_empty(), "Should have CMake errors");

    // Check for CMake warnings
    let cmake_warnings: Vec<_> = issues.iter()
        .filter(|i| i.code.as_ref().map_or(false, |c| c.contains("CMake Warning")))
        .collect();
    assert!(!cmake_warnings.is_empty(), "Should have CMake warnings");

    // Check for compiler errors (parsed by CppParser)
    let compiler_errors: Vec<_> = issues.iter()
        .filter(|i| matches!(i.level, IssueLevel::Error) && i.code.as_ref().map_or(true, |c| !c.contains("CMake")))
        .collect();

    println!("✓ CMake parser correctly parsed {} issues ({} CMake errors, {} CMake warnings, {} compiler issues)",
             issues.len(), cmake_errors.len(), cmake_warnings.len(), compiler_errors.len());
}

#[test]
fn test_cmake_error_parsing() {
    let output = r#"
CMake Error at CMakeLists.txt:10 (add_executable):
  Cannot find source file:
    src/main.cpp
"#;

    let parser = CMakeParser::new();
    let issues = parser.parse(output);

    assert_eq!(issues.len(), 1, "Should parse exactly 1 CMake error");

    let issue = &issues[0];
    assert!(matches!(issue.level, IssueLevel::Error));
    assert!(issue.location.file_path.contains("CMakeLists.txt"));
    assert_eq!(issue.location.line_number, Some(10));
    assert!(issue.message.contains("Cannot find source file"));
    assert!(issue.code.as_ref().unwrap().contains("CMake Error"));

    println!("✓ CMake error parsing works correctly");
}

#[test]
fn test_cmake_warning_parsing() {
    let output = r#"
CMake Warning at cmake/FindPackage.cmake:25 (find_package):
  Could not find a package configuration file
"#;

    let parser = CMakeParser::new();
    let issues = parser.parse(output);

    assert_eq!(issues.len(), 1, "Should parse exactly 1 CMake warning");

    let issue = &issues[0];
    assert!(matches!(issue.level, IssueLevel::Warning));
    assert!(issue.location.file_path.contains("FindPackage.cmake"));
    assert_eq!(issue.location.line_number, Some(25));
    assert!(issue.message.contains("Could not find"));
    assert!(issue.code.as_ref().unwrap().contains("CMake Warning"));

    println!("✓ CMake warning parsing works correctly");
}

#[test]
fn test_cmake_with_compiler_output() {
    let output = r#"
[ 50%] Building CXX object CMakeFiles/myapp.dir/src/main.cpp.o
src/main.cpp:10:5: error: 'x' was not declared in this scope
   10 |     int y = x + 1;
      |     ^~~~~
[ 75%] Building CXX object CMakeFiles/myapp.dir/src/utils.cpp.o
src/utils.cpp:25:12: warning: unused variable 'tmp' [-Wunused-variable]
"#;

    let parser = CMakeParser::new();
    let issues = parser.parse(output);

    assert_eq!(issues.len(), 2, "Should parse 2 compiler issues");

    // Check first error
    let error = &issues[0];
    assert!(matches!(error.level, IssueLevel::Error));
    assert!(error.location.file_path.contains("main.cpp"));

    // Check second warning
    let warning = &issues[1];
    assert!(matches!(warning.level, IssueLevel::Warning));
    assert!(warning.location.file_path.contains("utils.cpp"));

    println!("✓ CMake with compiler output parsing works correctly");
}

#[test]
fn test_cmake_is_issue_start() {
    let parser = CMakeParser::new();

    // CMake errors and warnings
    assert!(parser.is_issue_start("CMake Error at CMakeLists.txt:10:"));
    assert!(parser.is_issue_start("CMake Warning at cmake/module.cmake:25:"));

    // Compiler issues (GCC/Clang style)
    assert!(parser.is_issue_start("src/main.cpp:10:5: error:"));
    assert!(parser.is_issue_start("src/main.cpp:10:5: warning:"));

    // Compiler issues (MSVC style)
    assert!(parser.is_issue_start("src\\main.cpp(10,5): error C2065:"));
    assert!(parser.is_issue_start("src\\main.cpp(10,5): warning C4101:"));

    // Not issues
    assert!(!parser.is_issue_start("[ 50%] Building CXX object"));
    assert!(!parser.is_issue_start("   10 |     int x = 0;"));

    println!("✓ CMake is_issue_start detection works correctly");
}

#[test]
fn test_empty_output() {
    let parser = CMakeParser::new();
    let issues = parser.parse("");
    assert!(issues.is_empty(), "Empty output should produce no issues");

    println!("✓ Empty output handling works correctly");
}

#[test]
fn test_cmake_target_link_error() {
    let output = r#"
CMake Error at CMakeLists.txt:15 (target_link_libraries):
  Cannot specify link libraries for target "myapp" which is not built by
  this project.
"#;

    let parser = CMakeParser::new();
    let issues = parser.parse(output);

    assert_eq!(issues.len(), 1);
    let issue = &issues[0];
    assert!(matches!(issue.level, IssueLevel::Error));
    assert!(issue.location.file_path.contains("CMakeLists.txt"));
    assert_eq!(issue.location.line_number, Some(15));
    assert!(issue.message.contains("Cannot specify link libraries"));

    println!("✓ CMake target_link_libraries error parsing works correctly");
}

#[test]
fn test_cmake_find_package_warning() {
    let output = r#"
CMake Warning at cmake/FindBoost.cmake:42 (find_package):
  Could not find a package configuration file provided by "Boost" with any
  of the following names:
    BoostConfig.cmake
    boost-config.cmake
"#;

    let parser = CMakeParser::new();
    let issues = parser.parse(output);

    assert_eq!(issues.len(), 1);
    let issue = &issues[0];
    assert!(matches!(issue.level, IssueLevel::Warning));
    assert!(issue.location.file_path.contains("FindBoost.cmake"));
    assert_eq!(issue.location.line_number, Some(42));
    assert!(issue.message.contains("Could not find a package"));

    println!("✓ CMake find_package warning parsing works correctly");
}
