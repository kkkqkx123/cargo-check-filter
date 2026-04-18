//! 测试公共模块
//! 提供测试工具函数和共享逻辑

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// 获取项目根目录
pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// 获取测试输出目录
pub fn output_dir() -> PathBuf {
    let dir = project_root().join("test/output");
    fs::create_dir_all(&dir).expect("Failed to create output directory");
    dir
}

/// 获取 fixtures 目录
pub fn fixtures_dir() -> PathBuf {
    project_root().join("test/fixtures")
}

/// 保存命令输出到文件
pub fn save_output(name: &str, content: &str) -> PathBuf {
    let output_path = output_dir().join(format!("{}.txt", name));
    fs::write(&output_path, content).expect("Failed to write output file");
    println!("Output saved to: {}", output_path.display());
    output_path
}

/// 解析命令的完整路径（跨平台）
/// 在 Windows 上，会优先查找 .cmd, .bat, .exe 等可执行扩展名
pub fn resolve_command(cmd: &str) -> Option<PathBuf> {
    // 如果已经是路径，直接返回
    if cmd.contains('/') || cmd.contains('\\') {
        return Some(PathBuf::from(cmd));
    }

    // 使用 which/where 命令查找
    #[cfg(windows)]
    let check_cmd = "where";
    #[cfg(not(windows))]
    let check_cmd = "which";

    let output = Command::new(check_cmd).arg(cmd).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let paths: Vec<PathBuf> = stdout.lines().map(PathBuf::from).collect();

    #[cfg(windows)]
    {
        // 在 Windows 上，优先选择有扩展名的可执行文件
        // 优先级: .cmd > .bat > .exe > 其他
        let priority = [".cmd", ".bat", ".exe"];
        for ext in &priority {
            if let Some(path) = paths.iter().find(|p| {
                p.extension()
                    .map(|e| e.to_string_lossy().to_lowercase() == ext.trim_start_matches('.'))
                    .unwrap_or(false)
            }) {
                return Some(path.clone());
            }
        }
    }

    // 默认返回第一个找到的路径
    paths.into_iter().next()
}

/// 检查命令是否可用
pub fn is_command_available(cmd: &str) -> bool {
    resolve_command(cmd).is_some()
}

/// 运行命令并返回输出（使用解析后的完整路径）
pub fn run_command(cmd: &str, args: &[&str], cwd: &PathBuf) -> Result<String, String> {
    // 解析命令路径
    let cmd_path = resolve_command(cmd)
        .ok_or_else(|| format!("Command '{}' not found in PATH", cmd))?;

    println!("Executing: {} with args {:?}", cmd_path.display(), args);

    let output = Command::new(&cmd_path)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // 合并 stdout 和 stderr
    let full_output = if stderr.is_empty() {
        stdout.to_string()
    } else {
        format!("{}{}", stdout, stderr)
    };

    Ok(full_output)
}
