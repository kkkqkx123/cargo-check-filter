# Maven Compile 分析报告

**执行命令**: `mvn compile`

## 摘要

- **总问题数**: 23
- **错误数**: 8
- **警告数**: 15
- **信息数**: 0
- **涉及文件数**: 4

## 问题详情（按文件分组）

### D:\project\test\fixtures\maven-project\src\main\java\com\example\App.java

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 18 | 9 | Warning | [unchecked] unchecked conversion |

### D:\project\test\fixtures\maven-project\src\main\java\com\example\Utils.java

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 12 | 43 | Warning | [unchecked] unchecked conversion |
| 19 | 26 | Warning | [deprecation] getYear() in Date has been deprecated |
| 20 | 27 | Warning | [deprecation] getMonth() in Date has been deprecated |
| 21 | 25 | Warning | [deprecation] getDate() in Date has been deprecated |

### D:\project\test\fixtures\maven-project\src\main\java\com\example\Broken.java

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| 5 | 1 | Error | class Broken is public, should be declared in a file named Broken.java |
| 8 | 9 | Error | cannot find symbol |
| 12 | 16 | Error | cannot find symbol |
| 5 | 1 | Error | class Broken is public, should be declared in a file named Broken.java |
| 8 | 9 | Error | cannot find symbol |
| 12 | 16 | Error | cannot find symbol |

### pom.xml

| 行号 | 列号 | 级别 | 消息 |
|------|------|------|------|
| - | - | Warning | List rawList = new ArrayList(); |
| - | - | Warning | ^ |
| - | - | Warning | private static java.util.List rawList = new java.util.ArrayList(); |
| - | - | Warning | ^ |
| - | - | Warning | int year = date.getYear(); |
| - | - | Warning | ^ |
| - | - | Warning | int month = date.getMonth(); |
| - | - | Warning | ^ |
| - | - | Warning | int day = date.getDate(); |
| - | - | Warning | ^ |
| - | - | Error | Failed to execute goal org.apache.maven.plugins:maven-compiler-plugin:3.10.1:compile (default-compile) on project maven-test-project: Compilation failure: Compilation failure: |
| - | - | Error | -> [Help 1] |

## 原始输出

查看原始命令输出: [samples/maven_compile_sample.txt](samples/maven_compile_sample.txt)

---

*报告生成时间: 2026-04-18 19:34:37*
