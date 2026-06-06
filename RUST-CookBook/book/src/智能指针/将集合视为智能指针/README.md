# 将集合视为智能指针

> 本文整理自 Rust Design Patterns 的 Idiom：[Collections Are Smart Pointers](https://rust-unofficial.github.io/patterns/idioms/deref.html)。

归入“智能指针”，因为集合拥有底层数据，并通过借用暴露切片等视图。

## 待补充

- `Vec<T>`到`[T]`的Deref Coercion
- 拥有者与视图
- 配套Demo与实践检查表
