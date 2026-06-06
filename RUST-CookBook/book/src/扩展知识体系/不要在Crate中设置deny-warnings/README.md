# 不要在Crate中设置deny(warnings)

> 本文整理自 Rust Design Patterns 的 Anti-pattern：[`#[deny(warnings)]`](https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html)。

归入“扩展知识体系”，因为将所有Warning升级为错误会影响Crate跨编译器版本的兼容性。

## 待补充

- Crate源码与CI策略的区别
- 精确Lint级别
- 实践检查表
