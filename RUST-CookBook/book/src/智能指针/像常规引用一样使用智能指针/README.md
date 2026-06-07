# 像常规引用一样使用智能指针

> 《Rust 程序设计语言》：[像常规引用一样使用智能指针](https://doc.rust-lang.org/book/ch15-02-deref.html)。

## 实现 `Deref` 提供引用行为

{{#playground demo/src/bin/deref_实现自定义智能指针.rs editable}}

## 使用解引用强制转换简化调用

编译器可以将 `&String` 等智能指针引用转换为目标引用。

{{#playground demo/src/bin/deref_coercion_简化函数调用.rs editable}}

