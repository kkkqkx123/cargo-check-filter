//! NPM 集成测试
//! 执行实际的 npm 命令，验证解析逻辑与实际输出格式是否一致

use std::path::PathBuf;

mod common;
use common::{fixtures_dir, is_command_available, output_dir, resolve_command, run_command, save_output};

fn npm_project_path() -> PathBuf {
    fixtures_dir().join("npm-project")
}

/// 运行 npm install 安装依赖
fn ensure_npm_deps() -> Result<(), String> {
    if !is_command_available("npm") {
        return Err("npm is not available in PATH".to_string());
    }

    let project_path = npm_project_path();
    if !project_path.join("node_modules").exists() {
        println!("Installing npm dependencies...");
        match run_command("npm", &["install"], &project_path) {
            Ok(output) => {
                println!("npm install completed successfully");
                println!("{}", output);
            }
            Err(e) => {
                println!("npm install failed: {}", e);
                return Err(e);
            }
        }
    }
    Ok(())
}

#[test]
fn test_npm_eslint_output() {
    if let Err(e) = ensure_npm_deps() {
        println!("Skipping test: {}", e);
        return;
    }

    let project_path = npm_project_path();

    // 首先尝试使用 npx eslint
    let output = match run_command(
        "npx",
        &["eslint", "src/**/*.ts", "--format", "compact"],
        &project_path,
    ) {
        Ok(output) => output,
        Err(e) => {
            println!("npx eslint failed: {}, trying npm run lint...", e);
            // 回退到 npm run lint
            match run_command("npm", &["run", "lint"], &project_path) {
                Ok(output) => output,
                Err(e2) => {
                    panic!("Both npx eslint and npm run lint failed. npx error: {}, npm error: {}", e, e2);
                }
            }
        }
    };

    // 保存输出
    save_output("npm_eslint", &output);

    println!("=== NPM ESLint Output ===");
    println!("{}", output);

    // 验证输出中包含 ESLint 格式的问题
    // ESLint compact 格式: filepath:line:col: level message
    let lines: Vec<&str> = output.lines().collect();
    let has_issue_lines = lines.iter().any(|line| {
        line.contains(":") && (line.contains("error") || line.contains("warning"))
    });

    if has_issue_lines {
        println!("✓ Found ESLint issue lines in expected format");
    } else if output.contains("problem") || output.contains("issues") {
        println!("✓ ESLint completed (no issues or different format)");
    } else {
        println!("! No issue lines found (may be due to ESLint configuration)");
    }
}

#[test]
fn test_npm_typecheck_output() {
    if let Err(e) = ensure_npm_deps() {
        println!("Skipping test: {}", e);
        return;
    }

    let project_path = npm_project_path();

    // 使用 npx tsc
    let output = match run_command("npx", &["tsc", "--noEmit"], &project_path) {
        Ok(output) => output,
        Err(e) => {
            println!("npx tsc failed: {}, trying npm run type-check...", e);
            match run_command("npm", &["run", "type-check"], &project_path) {
                Ok(output) => output,
                Err(e2) => {
                    panic!("Both npx tsc and npm run type-check failed. npx error: {}, npm error: {}", e, e2);
                }
            }
        }
    };

    // 保存输出
    save_output("npm_typecheck", &output);

    println!("=== NPM TypeScript Type-Check Output ===");
    println!("{}", output);

    // TypeScript 错误格式: file(line,col): error TSxxxx: message
    let lines: Vec<&str> = output.lines().collect();
    let has_ts_errors = lines.iter().any(|line| {
        (line.contains("(") && line.contains(")") && line.contains("error"))
            || (line.contains(":") && line.contains("error TS"))
    });

    if has_ts_errors {
        println!("✓ Found TypeScript error lines in expected format");
    } else {
        println!("! No TypeScript errors found (may be due to strictness settings)");
    }
}

#[test]
fn test_npm_audit_output() {
    if !is_command_available("npm") {
        println!("Skipping test: npm is not available");
        return;
    }

    let project_path = npm_project_path();

    let output = match run_command("npm", &["audit"], &project_path) {
        Ok(output) => output,
        Err(e) => {
            panic!("Failed to run npm audit: {}", e);
        }
    };

    // 保存输出
    save_output("npm_audit", &output);

    println!("=== NPM Audit Output ===");
    println!("{}", output);

    // npm audit 输出包含漏洞信息
    assert!(
        output.contains("found")
            || output.contains("vulnerabilities")
            || output.contains("packages"),
        "Expected npm audit output format"
    );
}

#[test]
fn test_npm_ls_output() {
    if !is_command_available("npm") {
        println!("Skipping test: npm is not available");
        return;
    }

    let project_path = npm_project_path();

    let output = match run_command("npm", &["ls", "--depth=0"], &project_path) {
        Ok(output) => output,
        Err(e) => {
            println!("npm ls failed (this may be expected if deps not installed): {}", e);
            return;
        }
    };

    // 保存输出
    save_output("npm_ls", &output);

    println!("=== NPM List Output ===");
    println!("{}", output);
}

/// 验证 ESLint 输出格式
fn validate_eslint_output(content: &str) {
    println!("Validating ESLint output format...");
    let issue_lines: Vec<&str> = content
        .lines()
        .filter(|line| line.contains(":") && (line.contains("error") || line.contains("warning")))
        .collect();

    println!("  Found {} issue lines", issue_lines.len());

    for line in &issue_lines {
        // ESLint compact 格式: filepath:line:col: level message
        let parts: Vec<&str> = line.splitn(4, ':').collect();
        if parts.len() >= 3 {
            println!("  ✓ Valid format: {}", line);
        }
    }
}

/// 验证 TypeScript 输出格式
fn validate_typescript_output(content: &str) {
    println!("Validating TypeScript output format...");
    let issue_lines: Vec<&str> = content
        .lines()
        .filter(|line| line.contains("error TS"))
        .collect();

    println!("  Found {} error lines", issue_lines.len());
}

#[test]
fn test_validate_npm_outputs() {
    // 读取并验证已保存的 npm 输出文件
    let output_dir = output_dir();

    for entry in std::fs::read_dir(&output_dir).expect("Failed to read output directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            if filename.starts_with("npm_") && path.extension().map(|e| e == "txt").unwrap_or(false) {
                let content = std::fs::read_to_string(&path).expect("Failed to read output file");
                println!("Validating: {}", path.display());

                if filename.contains("eslint") {
                    validate_eslint_output(&content);
                } else if filename.contains("typecheck") {
                    validate_typescript_output(&content);
                }
            }
        }
    }
}

#[test]
fn test_npm_command_resolution() {
    // 测试命令解析功能
    println!("Testing npm command resolution...");
    
    if let Some(npm_path) = resolve_command("npm") {
        println!("✓ npm resolved to: {}", npm_path.display());
        assert!(npm_path.exists(), "Resolved npm path should exist");
    } else {
        println!("! npm not found in PATH");
    }

    if let Some(npx_path) = resolve_command("npx") {
        println!("✓ npx resolved to: {}", npx_path.display());
        assert!(npx_path.exists(), "Resolved npx path should exist");
    } else {
        println!("! npx not found in PATH");
    }
}
