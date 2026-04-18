# ESLint 分析报告

**执行命令**: `npm run lint`

## 摘要

- **总问题数**: 12
- **错误数**: 4
- **警告数**: 8
- **信息数**: 0
- **涉及文件数**: 2

## 问题详情（按文件分组）

### D:\项目\cli\analyze-cargo\tests\data\fixtures\npm-project\src\index.ts

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 3 | 7 | Warning | 'unusedVariable' is assigned a value but never used |
| 10 | 5 | Error | Unexpected console statement |
| 13 | 7 | Error | Argument of type 'number' is not assignable to parameter of type 'string' |
| 17 | 10 | Warning | 'unusedFunction' is defined but never used |
| 20 | 23 | Warning | Parameter 'data' implicitly has an 'any' type |
| 26 | 1 | Warning | Missing return type on function |
| 30 | 1 | Error | Unexpected var, use let or const instead |

### D:\项目\cli\analyze-cargo\tests\data\fixtures\npm-project\src\utils.ts

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 4 | 10 | Error | 'readFileSync' is defined but never used |
| 15 | 29 | Warning | Parameter 'data' implicitly has an 'any' type |
| 20 | 24 | Warning | Parameter 'users' implicitly has an 'any' type |
| 20 | 37 | Warning | Return type annotation is missing |
| 23 | 7 | Warning | 'API_URL' is assigned a value but never used |

## 原始输出

查看原始命令输出: [samples/npm_eslint_sample.txt](samples/npm_eslint_sample.txt)

---

*报告生成时间: 2026-04-18 19:34:37*
