# 在Destructor中完成资源清理

> 本文整理自 Rust Design Patterns 的 Idiom：[Finalisation in Destructors](https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html)。

归入“智能指针”，因为Destructor与Drop负责在资源生命周期结束时执行清理。

## 待补充

- Drop中的清理
- Drop不能返回错误
- 配套Demo与实践检查表
