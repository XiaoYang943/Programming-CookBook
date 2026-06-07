# Slice 类型

> 《Rust 程序设计语言》原文：[Slice 类型](https://doc.rust-lang.org/book/ch04-03-slices.html)。

## 使用 `&str` 借用字符串片段



字符串 Slice `&str` 借用字符串中的连续 UTF-8 字节区间，不拥有字符串数据。

{{#playground demo/src/bin/string_slice_借用字符串片段.rs editable}}

**语法限制**

字符串 Slice 的边界必须位于有效的 UTF-8 字符边界，否则运行时会 panic。

{{#playground demo/src/bin/string_slice_UTF8边界panic.rs editable should_panic}}

## 使用 `&[T]` 借用连续元素



Slice `&[T]` 可以借用数组、`Vec<T>` 等连续集合的一部分，并记录起始地址和长度。

{{#playground demo/src/bin/slice_借用连续元素.rs editable}}

## 使用 Slice 返回原数据片段



返回 Slice 可以让结果与原数据保持关联，避免单独维护索引或复制数据。

{{#playground demo/src/bin/first_word_返回字符串Slice.rs editable}}

## Slice 实践

- 只需要读取连续数据时，函数参数优先使用 `&str` 或 `&[T]`。
- Slice 不拥有数据，不能比原数据存活更久。
- 不要使用字节索引随意切割 UTF-8 字符串。
