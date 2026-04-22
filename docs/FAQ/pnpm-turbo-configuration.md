# PNPM + Turbo 配置指南

本文档说明如何配置 PNPM + Turbo 项目以支持 analyzer 正确解析 lint 输出。

## 问题描述

Turbo 默认使用 TUI (Terminal User Interface) 模式，该模式会输出大量的 ANSI 控制序列（如光标定位、颜色代码等）。这些控制序列会干扰 analyzer 的解析，导致：

1. 解析不到任何 issues（报告为空）
2. 文件路径被错误识别
3. 规则名称被截断或合并

## 解决方案

### 1. 修改 turbo.json

在项目的 `turbo.json` 文件中添加 `"ui": "stream"` 配置：

```json
{
  "$schema": "https://turbo.build/schema.json",
  "ui": "stream",
  "tasks": {
    "lint": {
      "dependsOn": ["^lint"]
    }
  }
}
```

### 2. 清理 Turbo 缓存

修改配置后，需要清理缓存以确保新的输出格式生效：

```bash
# 清理 turbo 缓存
pnpm exec turbo daemon clean

# 或者删除缓存目录
rm -rf node_modules/.cache/turbo

# 重新运行 lint
pnpm lint
```

### 3. 验证配置

运行 analyzer 验证是否能正确解析：

```bash
cd D:\项目\cli\analyzer
cargo test --test verify_fix test_real_project_parsing -- --nocapture
```

如果配置正确，测试应该通过并显示类似以下输出：

```
=== Parsing Results ===
Total issues found: 1614
✓ All issues have correct file paths
test test_real_project_parsing ... ok
```

## 配置说明

### `ui` 选项

- `"tui"` (默认): 使用交互式终端界面，输出包含大量控制序列
- `"stream"`: 使用流式输出，每行一个日志条目，易于解析

### 为什么需要 stream 模式

| 模式   | 输出特点                           | 解析难度 |
| ------ | ---------------------------------- | -------- |
| TUI    | 包含光标定位、清屏、颜色等控制序列 | 困难     |
| Stream | 纯文本，每行一个前缀+内容          | 简单     |

TUI 模式的输出示例：

```
@graph-agent/sdk:lint: [?25l[2J[m[H[4m
@graph-agent/sdk:lint: D:\项目\agent\graph-agent\sdk\agent\checkpoint\agent-loop-delta-restorer.ts[24m
@graph-agent/sdk:lint:   [2m25:15[22m  [33mwarning  [mUnexpected any...
```

Stream 模式的输出示例：

```
@graph-agent/sdk:lint:
@graph-agent/sdk:lint: D:\项目\agent\graph-agent\sdk\agent\checkpoint\agent-loop-delta-restorer.ts
@graph-agent/sdk:lint:   25:15  warning  Unexpected any. Specify a different type  @typescript-eslint/no-explicit-any
```

## 注意事项

1. **缓存问题**: 修改 `turbo.json` 后必须清理缓存，否则旧的 TUI 格式输出仍会被使用
2. **团队协作**: 建议将 `"ui": "stream"` 提交到版本控制，确保团队成员使用一致的配置
3. **CI/CD**: Stream 模式也更适合 CI/CD 环境，因为 TUI 在非交互式环境中可能无法正常显示

## 故障排除

### 问题：修改配置后仍然解析失败

**可能原因**: 缓存未完全清理

**解决方案**:

```bash
# 停止 turbo daemon
pnpm exec turbo daemon stop

# 清理所有缓存
rm -rf node_modules/.cache/turbo
rm -rf .turbo

# 重新运行
pnpm lint
```

### 问题：部分包解析正常，部分包失败

**可能原因**: 部分包的缓存是在 TUI 模式下生成的

**解决方案**: 使用 `--force` 标志强制重新运行所有任务

```bash
pnpm exec turbo run lint --force
```

## 参考链接

- [Turbo 配置文档](https://turbo.build/repo/docs/reference/configuration#ui)
- [Turbo Run 命令文档](https://turbo.build/repo/docs/reference/run#--ui)
