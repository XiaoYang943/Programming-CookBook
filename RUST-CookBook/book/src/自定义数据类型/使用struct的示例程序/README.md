# 使用 struct 的示例程序

> 《Rust 程序设计语言》原文：[使用 struct 的示例程序](https://doc.rust-lang.org/book/ch05-02-example-structs.html)。

## 使用 struct 表达领域概念

将宽度和高度组织为 `Rectangle`，可以避免重复参数并让函数签名表达真实含义。

{{#playground demo/src/bin/rectangle_使用struct重构参数.rs editable}}

## 使用派生 trait 辅助调试与比较

`#[derive(Debug, PartialEq)]` 可以为类型自动实现常用 trait，便于调试和断言。

{{#playground demo/src/bin/derive_派生Debug与PartialEq.rs editable}}

