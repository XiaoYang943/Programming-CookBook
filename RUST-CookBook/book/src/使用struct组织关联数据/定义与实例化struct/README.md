# 定义与实例化 struct

> 《Rust 程序设计语言》原文：[定义与实例化 struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)。

## 使用命名字段定义 struct

**核心规则**

命名字段 struct 为相关数据提供明确字段名。实例默认整体不可变，字段可变性由实例的 `mut` 决定。

{{#playground demo/src/bin/named_struct_定义与修改字段.rs editable}}

## 使用字段初始化简写

当变量名与字段名相同时，可以省略重复的 `field: field`。

{{#playground demo/src/bin/field_init_字段初始化简写.rs editable}}

## 使用 struct 更新语法复用字段

`..other` 会移动或复制未显式指定的字段；使用后原实例是否仍可用取决于被移动字段是否实现 `Copy`。

{{#playground demo/src/bin/update_syntax_复用实例字段.rs editable}}

{{#playground snippets/update_syntax_移动字段后原实例失效.rs editable ignore mdbook-runnable}}

## 使用 Tuple struct 与 Unit-like struct

Tuple struct 适合字段含义由位置表达的轻量封装；Unit-like struct 适合没有字段但需要独立类型的场景。

{{#playground demo/src/bin/tuple_unit_struct_轻量类型.rs editable}}

