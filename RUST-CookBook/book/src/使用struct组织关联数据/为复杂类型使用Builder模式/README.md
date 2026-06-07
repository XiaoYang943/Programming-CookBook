# 为复杂类型使用Builder模式

> 本文对应 Effective Rust **Item 7**：[Use builders for complex types](https://effective-rust.com/builders.html)。

## 使用 Builder 分步骤配置复杂类型

Builder 适合可选参数较多、需要链式配置或构造过程可能失败的类型。

{{#playground demo/src/bin/builder_分步骤构造复杂类型.rs editable}}

## 在 `build` 中验证必填字段

Builder 可以暂存不完整状态，并在 `build` 时返回 `Result`。

{{#playground demo/src/bin/builder_build_验证必填字段.rs editable}}

## Builder 使用原则

- 简单类型优先直接使用 struct 表达式或 `new`。
- Setter 返回 `Self` 可以支持链式调用。
- 构造可能失败时，让 `build` 返回 `Result`。
