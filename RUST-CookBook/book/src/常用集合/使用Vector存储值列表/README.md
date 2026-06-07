# 使用 Vector 存储值列表

> 《Rust 程序设计语言》原文：[使用 Vector 存储值列表](https://doc.rust-lang.org/book/ch08-01-vectors.html)。

## 使用 `Vec<T>` 存储可变长度列表

{{#playground demo/src/bin/vector_创建并追加元素.rs editable}}

## 使用索引或 `get` 读取元素

索引越界会 panic；`get` 返回 `Option<&T>`，适合处理不确定索引。

{{#playground demo/src/bin/vector_get_安全读取元素.rs editable}}

## 遍历并修改元素

{{#playground demo/src/bin/vector_iter_遍历并修改元素.rs editable}}

## 避免在持有元素引用时修改 Vector

追加元素可能重新分配内存，因此持有元素引用时不能同时修改 Vector。

{{#playground snippets/vector_持有引用时不能追加元素.rs editable ignore mdbook-runnable}}

