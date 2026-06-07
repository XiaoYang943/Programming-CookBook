# 将集合视为智能指针

> 本文整理自 Rust Design Patterns 的 Idiom：[Collections Are Smart Pointers](https://rust-unofficial.github.io/patterns/idioms/deref.html)。

## 将拥有集合与借用视图分开

`Vec<T>` 和 `String` 拥有底层数据，并能通过解引用强制转换暴露 `&[T]` 和 `&str` 视图。

{{#playground demo/src/bin/collection_拥有者与借用视图.rs editable}}

## 函数参数优先接收借用视图

{{#playground demo/src/bin/slice_集合通过Slice传参.rs editable}}
