# 定义 enum

> 《Rust 程序设计语言》：[定义 enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)。

## 使用 `enum` 表达有限状态

{{#playground demo/src/bin/enum_表达有限状态.rs editable}}

## 为 enum 变体携带不同数据

每个变体可以携带不同类型和数量的数据，使状态与其有效数据保持一致。

{{#playground demo/src/bin/variant_data_变体携带不同数据.rs editable}}

## 使用 `Option<T>` 表达值可能缺失

`Option<T>` 使用 `Some(T)` 和 `None` 代替空值。

{{#playground demo/src/bin/option_表达值可能缺失.rs editable}}

