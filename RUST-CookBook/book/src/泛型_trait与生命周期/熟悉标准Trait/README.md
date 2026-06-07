# 熟悉标准 trait

> 本文对应 Effective Rust **Item 10**：[Familiarize yourself with standard traits](https://effective-rust.com/std-traits.html)。

## 实现 `Debug`、`Clone` 与 `Default`

常用标准 trait 让自定义类型融入调试、复制和默认构造流程，能派生时优先使用 `derive`。

{{#playground demo/src/bin/derive_实现常用标准trait.rs editable}}

## 实现 `From` 提供可靠转换

{{#playground demo/src/bin/from_实现可靠转换.rs editable}}

## 实现 `Iterator` 接入迭代器生态

{{#playground demo/src/bin/iterator_实现迭代器协议.rs editable}}

## 标准 trait 实现原则

- 仅在语义满足契约时实现标准 trait。
- `Eq`、`Ord` 与 `Hash` 等实现必须保持一致。
- 优先实现 `From`，自动获得对应 `Into`。
