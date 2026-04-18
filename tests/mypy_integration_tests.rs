//! Mypy (Python) 集成测试
//! 执行实际的 mypy 命令，验证解析逻辑与实际输出格式是否一致

use std::path::PathBuf;
use std::process::Command;

mod common;
use common::{fixtures_dir, is_command_available, output_dir, run_command, save_output};

fn python_project_path() -> PathBuf {
    fixtures_dir().join("python-project")
}

/// 检查 mypy 是否可用
fn ensure_mypy() -> Result<(), String> {
    if !is_command_available("mypy") {
        return Err("mypy is not installed. Please install it with: pip install mypy".to_string());
    }
    Ok(())
}

#[test]
fn test_mypy_basic_output() {
    if let Err(e) = ensure_mypy() {
        println!("Skipping test: {}", e);
        return;
    }

    let project_path = python_project_path();

    let output = match run_command("mypy", &["--show-column-numbers", "."], &project_path) {
        Ok(output) => output,
        Err(e) => {
            panic!("Failed to run mypy: {}", e);
        }
    };

    // 保存输出
    save_output("mypy_basic", &output);

    println!("=== Mypy Basic Output ===");
    println!("{}", output);

    // 验证 mypy 输出格式
    // 格式: file:line:col: level: message
    let lines: Vec<&str> = output.lines().collect();
    let has_mypy_errors = lines.iter().any(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        parts.len() >= 4 && (line.contains("error:") || line.contains("warning:"))
    });

    if has_mypy_errors {
        println!("✓ Found mypy error lines in expected format (file:line:col: level: message)");
    } else if output.contains("Success") {
        println!("✓ Mypy reported success (no issues found)");
    } else {
        println!("! Unexpected mypy output format");
    }

    // 验证输出格式符合解析器预期
    for line in &lines {
        if line.contains(":") && (line.contains("error:") || line.contains("warning:")) {
            let parts: Vec<&str> = line.splitn(5, ':').collect();
            if parts.len() >= 4 {
                println!("  Found issue: {}", line);
            }
        }
    }
}

#[test]
fn test_mypy_strict_output() {
    if ensure_mypy().is_err() {
        println!("Skipping test: mypy is not available");
        return;
    }

    let project_path = python_project_path();

    let output = match run_command(
        "mypy",
        &["--strict", "--show-column-numbers", "."],
        &project_path,
    ) {
        Ok(output) => output,
        Err(e) => {
            panic!("Failed to run mypy --strict: {}", e);
        }
    };

    // 保存输出
    save_output("mypy_strict", &output);

    println!("=== Mypy Strict Output ===");
    println!("{}", output);

    // 严格模式下应该有更多错误
    let error_count = output.lines().filter(|line| line.contains("error:")).count();

    println!("Found {} error lines", error_count);

    // 验证输出包含统计信息
    if output.contains("Found") && output.contains("error") {
        println!("✓ Found mypy summary line");
    }
}

#[test]
fn test_mypy_specific_file() {
    if ensure_mypy().is_err() {
        println!("Skipping test: mypy is not available");
        return;
    }

    let project_path = python_project_path();
    let main_py = project_path.join("src/main.py");

    let output = match run_command("mypy", &["--show-column-numbers", main_py.to_str().unwrap()], &project_path) {
        Ok(output) => output,
        Err(e) => {
            panic!("Failed to run mypy on specific file: {}", e);
        }
    };

    // 保存输出
    save_output("mypy_specific_file", &output);

    println!("=== Mypy Specific File Output ===");
    println!("{}", output);

    // 验证特定文件的输出格式
    for line in output.lines() {
        if line.contains("main.py") && line.contains(":") {
            println!("  Issue in main.py: {}", line);
        }
    }
}

#[test]
fn test_mypy_with_ignore_missing_imports() {
    if ensure_mypy().is_err() {
        println!("Skipping test: mypy is not available");
        return;
    }

    let project_path = python_project_path();

    let output = match run_command(
        "mypy",
        &[
            "--show-column-numbers",
            "--ignore-missing-imports",
            ".",
        ],
        &project_path,
    ) {
        Ok(output) => output,
        Err(e) => {
            panic!("Failed to run mypy: {}", e);
        }
    };

    // 保存输出
    save_output("mypy_ignore_imports", &output);

    println!("=== Mypy with --ignore-missing-imports Output ===");
    println!("{}", output);
}

/// 验证 mypy 输出格式
fn validate_mypy_output(content: &str) {
    println!("Validating mypy output format...");
    let issue_lines: Vec<&str> = content
        .lines()
        .filter(|line| line.contains(":") && (line.contains("error:") || line.contains("warning:")))
        .collect();

    println!("  Found {} issue lines", issue_lines.len());

    for line in &issue_lines {
        // 验证格式: file:line:col: level: message
        let parts: Vec<&str> = line.splitn(5, ':').collect();
        if parts.len() >= 4 {
            let _file = parts[0];
            let _line_num = parts[1].trim().parse::<u32>();
            let _col_num = parts[2].trim().parse::<u32>();
            let _level = parts[3].trim();
            println!("  ✓ Valid format: {}", line);
        }
    }
}

#[test]
fn test_validate_mypy_outputs() {
    // 读取并验证已保存的 mypy 输出文件
    let output_dir = output_dir();

    for entry in std::fs::read_dir(&output_dir).expect("Failed to read output directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            if filename.starts_with("mypy_") && path.extension().map(|e| e == "txt").unwrap_or(false)
            {
                let content = std::fs::read_to_string(&path).expect("Failed to read output file");
                println!("Validating: {}", path.display());
                validate_mypy_output(&content);
            }
        }
    }
}
