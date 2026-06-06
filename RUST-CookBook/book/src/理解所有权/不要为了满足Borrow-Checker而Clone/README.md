# 不要为了满足Borrow Checker而Clone

> 本文整理自 Rust Design Patterns 的 Anti-pattern：[Clone to satisfy the borrow checker](https://rust-unofficial.github.io/patterns/anti_patterns/borrow_clone.html)。

归入“理解所有权”，因为为了绕过借用问题而随意Clone会隐藏所有权设计问题并增加运行成本。

## 待补充

- 常见错误表现
- 调整借用范围和数据结构
- 配套Demo与检查表
