# 不要使用 `Deref` 模拟继承

> 本文整理自 Rust Design Patterns 的 Anti-pattern：[Deref Polymorphism](https://rust-unofficial.github.io/patterns/anti_patterns/deref.html)。

## 仅为指针语义实现 `Deref`

`Deref` 表达智能指针到目标值的透明访问，不应仅为了复用目标类型的方法而实现。

## 使用组合与 trait 表达共同行为

{{#playground demo/src/bin/trait_使用共同行为代替Deref继承.rs editable}}

## 检查表

- 类型是否真正拥有或透明包装目标值？
- 自动方法解析是否会让 API 边界变得模糊？
- 使用明确方法或 trait 是否更清晰？
