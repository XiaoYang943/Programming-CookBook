# 使用借用类型作为函数参数

> 本文整理自 Rust Design Patterns 的 Idiom：[Use borrowed types for arguments](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)。

## 使用 `&str` 代替 `&String`

**核心规则**

只需要读取字符串内容时，参数使用 `&str`。它既能接收 `String` 的借用，也能接收字符串字面量和其他字符串 Slice。

{{#playground demo/src/bin/str_使用借用字符串参数.rs editable}}

## 使用 `&[T]` 代替 `&Vec<T>`

**核心规则**

只需要读取连续元素时，参数使用 `&[T]`。它可以接收数组、`Vec<T>` 及其局部 Slice。

{{#playground demo/src/bin/slice_使用借用序列参数.rs editable}}

## 使用 `&mut [T]` 修改连续元素

**核心规则**

函数需要修改元素但不需要改变集合长度时，使用 `&mut [T]`，避免将 API 限制为 `Vec<T>`。

{{#playground demo/src/bin/mutable_slice_修改连续元素.rs editable}}

## 参数设计原则

| 函数需求 | 参数类型 |
| --- | --- |
| 读取字符串 | `&str` |
| 读取连续元素 | `&[T]` |
| 修改连续元素但不改变长度 | `&mut [T]` |
| 获取并保存所有权 | `String`、`Vec<T>` 等拥有类型 |
