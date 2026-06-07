# 什么是所有权？

> 《Rust 程序设计语言》原文：[什么是所有权？](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)。

## 在作用域结束时自动执行 `drop`

**核心规则**

值的所有者离开作用域时，Rust 会自动调用 `drop` 释放该值持有的资源。

{{#playground demo/src/bin/scope_drop_作用域结束释放资源.rs editable}}

## 使用移动转移所有权

**核心规则**

将拥有堆资源的值赋给另一个变量时，所有权通常会移动到新变量，旧变量随后失效。

{{#playground demo/src/bin/move_转移所有权.rs editable}}

**语法限制**

移动后继续使用旧变量会导致编译失败。

{{#playground snippets/move_移动后不能使用旧变量.rs editable ignore mdbook-runnable}}

## 使用 `clone` 深度复制数据

**核心规则**

需要两个彼此独立的拥有者时，可以显式调用 `clone` 复制数据及其堆资源。

{{#playground demo/src/bin/clone_深度复制数据.rs editable}}

## 使用 `Copy` 复制栈上值

**核心规则**

实现 `Copy` 的类型在赋值和传参时会复制值，旧变量仍然有效。整数、布尔值和完全由 `Copy` 类型组成的元组通常实现了 `Copy`。

{{#playground demo/src/bin/copy_复制栈上值.rs editable}}

## 通过参数与返回值转移所有权

**核心规则**

按值传参会移动或复制参数；返回值可以将所有权交给调用者。只需要临时访问值时，应使用引用。

{{#playground demo/src/bin/function_通过参数和返回值转移所有权.rs editable}}

## 选择移动、`clone` 或借用

| 需求 | 选择 |
| --- | --- |
| 调用方不再需要值 | 移动所有权 |
| 双方需要独立拥有数据 | `clone` |
| 只需要临时读取或修改值 | 借用 |
| 类型实现 `Copy` 且复制成本低 | 按值复制 |
