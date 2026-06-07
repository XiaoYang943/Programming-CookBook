# 理解类型转换

> 本文对应 Effective Rust **Item 5**：[Understand type conversions](https://effective-rust.com/casts.html)。

## 使用 `from` 与 `into` 执行可靠转换



当转换始终成功且不会丢失信息时，实现或使用 `From<T>`。对应的 `Into<U>` 会自动获得。

{{#playground demo/src/bin/from_into_可靠类型转换.rs editable}}

## 使用 `try_from` 与 `try_into` 执行可失败转换



转换可能失败时使用 `TryFrom` 或 `TryInto`，通过 `Result` 显式处理失败原因。

{{#playground demo/src/bin/try_from_可失败类型转换.rs editable}}

## 谨慎使用 `as` 转换



`as` 可以执行数值转换，但可能截断、改变符号或损失精度，并且不会返回错误。

{{#playground demo/src/bin/as_cast_可能丢失信息.rs editable}}

**选择原则**

| 转换性质 | 优先选择 |
| --- | --- |
| 始终成功且无信息损失 | `From` / `Into` |
| 可能失败或越界 | `TryFrom` / `TryInto` |
| 明确接受底层数值 Cast 语义 | `as` |

**实践建议**

- API 参数通常接收 `impl Into<T>`，实现中优先定义 `From<T>`。
- 边界处优先使用 `TryFrom`，不要让越界被静默截断。
- 使用 `as` 时明确记录并测试其截断或精度损失行为。
