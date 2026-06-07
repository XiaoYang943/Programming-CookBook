# 使用自定义 trait 简化复杂类型约束

> 本文整理自 Rust Design Patterns：[Avoid complex type bounds with custom traits](https://rust-unofficial.github.io/patterns/patterns/structural/trait-for-bounds.html)。

## 使用自定义 trait 聚合约束

稳定 Rust 尚未提供 trait alias，可以定义空 trait 并使用 blanket implementation 聚合重复约束。

{{#playground demo/src/bin/custom_trait_聚合复杂约束.rs editable}}

## 使用 blanket implementation 自动实现

{{#playground demo/src/bin/blanket_impl_为满足约束的类型自动实现.rs editable}}

## 使用原则

- 约束只出现一两次时直接使用 `where` 更清晰。
- 多个 API 反复共享同一组约束时，再定义自定义 trait。
- 自定义 trait 名称应表达能力，而不是仅描述实现细节。
