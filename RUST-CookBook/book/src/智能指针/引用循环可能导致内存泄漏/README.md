# 引用循环可能导致内存泄漏

> 《Rust 程序设计语言》原文：[引用循环可能导致内存泄漏](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)。

## 理解 `Rc<T>` 引用循环

强引用形成环时，引用计数不会降为零，值无法释放。

{{#playground demo/src/bin/rc_cycle_强引用循环.rs editable}}

## 使用 `Weak<T>` 表达非拥有关系

`Weak<T>` 不增加强引用计数，适合父节点、缓存或观察者等非拥有关系。

{{#playground demo/src/bin/weak_打破引用循环.rs editable}}

