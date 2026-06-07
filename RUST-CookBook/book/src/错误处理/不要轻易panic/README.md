# 不要轻易 `panic`

> 本文对应 Effective Rust **Item 18**：[Don't panic](https://effective-rust.com/panic.html)。

## 使用 `Result` 保留调用方选择权

公共 API 遇到可预期失败时返回 `Result`，不要替调用方决定终止程序。

{{#playground demo/src/bin/result_公共API返回错误.rs editable}}

## 为可能 panic 的 API 记录契约

索引、断言和违反前置条件等 API 可能 panic。公共接口应在文档中清楚说明 panic 条件。

## `panic` 检查表

- 错误是否可由调用方恢复？
- 是否来自外部输入或环境？
- 是否能返回包含上下文的 Error？
- panic 条件是否属于明确的程序不变量？
