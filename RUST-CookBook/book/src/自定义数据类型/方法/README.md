# 方法

> 《Rust 程序设计语言》原文：[方法](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)。

## 使用 `impl` 定义方法

方法的第一个参数是 `self`、`&self` 或 `&mut self`。只读取实例时优先使用 `&self`。

{{#playground demo/src/bin/method_使用self访问实例.rs editable}}

## 使用 `&mut self` 修改实例

{{#playground demo/src/bin/mutable_method_修改实例状态.rs editable}}

## 使用关联函数创建命名入口

不接收 `self` 的函数称为关联函数，通过 `Type::function()` 调用。

{{#playground demo/src/bin/associated_function_关联函数.rs editable}}

