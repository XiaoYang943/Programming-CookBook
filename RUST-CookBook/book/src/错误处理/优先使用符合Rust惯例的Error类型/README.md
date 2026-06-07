# 优先使用符合 Rust 惯例的 Error 类型

> Effective Rust **Item 4**：[Prefer idiomatic Error types](https://effective-rust.com/errors.html)。

## 为库定义具体 Error 类型

库错误应表达调用方可处理的失败类别，并实现 `Display` 与 `std::error::Error`。

{{#playground demo/src/bin/error_type_实现Display与Error.rs editable}}

## 使用 `source` 保留底层错误

包装错误时保留原始来源，便于诊断错误链。

{{#playground demo/src/bin/error_source_保留底层错误.rs editable}}

## Error 类型设计原则

- 库优先提供具体、稳定的错误类型。
- 应用边界可以使用更灵活的动态错误。
- 错误信息用于人类阅读，错误变体用于程序处理。
