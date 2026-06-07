# 使用 `format!` 拼接 `String`

> 本文整理自 Rust Design Patterns 的 Idiom：[Concatenating Strings with `format!`](https://rust-unofficial.github.io/patterns/idioms/concat-format.html)。

## 使用 `format!` 组合多个值

`format!` 不取得参数所有权，并返回新的 `String`。

{{#playground demo/src/bin/format_组合多个值.rs editable}}

## 使用 `push_str` 原地追加文本

已有可变 `String` 且需要逐步构建时，使用 `push` 或 `push_str`。

{{#playground demo/src/bin/push_str_原地追加文本.rs editable}}

## 选择拼接方式

| 场景 | 选择 |
| --- | --- |
| 一次性组合多个值，重视可读性 | `format!` |
| 原地逐步构建字符串 | `push` / `push_str` |
