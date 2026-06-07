# 使用 trait 定义共同行为

> 《Rust 程序设计语言》：[使用 trait 定义共同行为](https://doc.rust-lang.org/book/ch10-02-traits.html)。

## 使用 trait 定义行为契约

trait 声明类型必须提供的方法，让不同类型能够共享行为接口。

{{#playground demo/src/bin/trait_定义并实现共同行为.rs editable}}

## 使用 trait bound 约束泛型

{{#playground demo/src/bin/trait_bound_约束泛型参数.rs editable}}

## 使用 `where` 简化复杂约束

{{#playground demo/src/bin/where_clause_整理类型约束.rs editable}}

## 使用 `impl Trait` 简化参数与返回类型

参数位置的 `impl Trait` 是泛型参数的简写；返回位置用于隐藏具体返回类型，但单个返回路径必须保持同一具体类型。

{{#playground demo/src/bin/impl_trait_简化参数与返回类型.rs editable}}

{{#playground snippets/impl_trait_不能返回不同具体类型.rs editable ignore mdbook-runnable}}

## 理解孤儿规则

只有 trait 或被实现类型至少有一个定义在当前 crate 中时，才能实现该 trait。

