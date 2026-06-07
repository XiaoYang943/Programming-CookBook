# 构造函数惯例

> 本文整理自 Rust Design Patterns 的 Idiom：[Constructor](https://rust-unofficial.github.io/patterns/idioms/ctor.html)。

## 使用 `new` 提供主要构造入口

Rust 没有专门的构造函数语法，通常使用返回 `Self` 的关联函数 `new`。

{{#playground demo/src/bin/new_提供主要构造入口.rs editable}}

## 使用命名关联函数表达不同语义

存在多种常用构造方式时，使用 `from_*`、`with_*` 等名称明确区别。

{{#playground demo/src/bin/named_constructor_提供多种构造入口.rs editable}}
