# 栈上动态分派 On-Stack Dynamic Dispatch

> 本文整理自 Rust Design Patterns 的 Idiom：[On-Stack Dynamic Dispatch](https://rust-unofficial.github.io/patterns/idioms/on-stack-dyn-dispatch.html)。

归入“泛型、trait与生命周期”，因为它使用Trait Object实现动态分派，同时避免不必要的堆分配。

## 待补充

- `&dyn Trait`
- 与`Box<dyn Trait>`的权衡
- 配套Demo与实践检查表
