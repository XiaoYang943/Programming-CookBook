# 不要使用Deref模拟继承

> 本文整理自 Rust Design Patterns 的 Anti-pattern：[Deref Polymorphism](https://rust-unofficial.github.io/patterns/anti_patterns/deref.html)。

归入“智能指针”，因为滥用Deref模拟继承会混淆智能指针语义和API边界。

## 待补充

- Deref的适用场景
- 组合与Trait替代方案
- 配套Demo与检查表
