# TypeScript Type Check 分析报告

**执行命令**: `npm run type-check`

## 摘要

- **总问题数**: 7
- **错误数**: 7
- **警告数**: 0
- **信息数**: 0
- **涉及文件数**: 2

## 问题详情（按文件分组）

### src/utils.ts

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 15 | 29 | Error | Parameter 'data' implicitly has an 'any' type. |
| 20 | 24 | Error | Parameter 'users' implicitly has an 'any' type. |
| 20 | 37 | Error | 'processUsers', which lacks return-type annotation, implicitly has an 'any' return type. |
| 53 | 5 | Error | Rest parameter 'args' implicitly has an 'any[]' type. |

### src/index.ts

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 13 | 7 | Error | Argument of type 'number' is not assignable to parameter of type 'string'. |
| 20 | 23 | Error | Parameter 'data' implicitly has an 'any' type. |
| 26 | 1 | Error | 'calculate', which lacks return-type annotation, implicitly has an 'any' return type. |

## 原始输出

查看原始命令输出: [samples/npm_typecheck_sample.txt](samples/npm_typecheck_sample.txt)

---

*报告生成时间: 2026-04-18 19:34:37*
