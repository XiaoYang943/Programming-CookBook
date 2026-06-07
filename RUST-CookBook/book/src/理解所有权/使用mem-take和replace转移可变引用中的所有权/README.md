# 使用 `mem::take` 和 `mem::replace` 转移可变引用中的所有权

> 本文整理自 Rust Design Patterns 的 Idiom：[`mem::{take(_), replace(_)}`](https://rust-unofficial.github.io/patterns/idioms/mem-replace.html)。

## 使用 `mem::take` 取出值并留下默认值

**核心规则**

不能直接从可变引用后面移动字段。字段类型实现 `Default` 时，可以使用 `mem::take` 取出旧值并留下默认值。

{{#playground demo/src/bin/mem_take_取出值并留下默认值.rs editable}}

## 使用 `mem::replace` 取出值并留下指定值

**核心规则**

需要自定义替代值时，使用 `mem::replace` 原子地取出旧值并写入新值。

{{#playground demo/src/bin/mem_replace_取出值并留下指定值.rs editable}}

## 避免从可变引用后移动字段

**语法限制**

直接移动可变引用中的非 `Copy` 字段会让原值处于不完整状态，因此编译器会拒绝。

{{#playground snippets/move_out_不能直接移出可变引用字段.rs editable ignore mdbook-runnable}}

## 选择 `take` 或 `replace`

| 需求 | 选择 |
| --- | --- |
| 替代值使用 `Default::default()` | `mem::take` |
| 需要指定替代值 | `mem::replace` |
| 只需要交换两个现有值 | `mem::swap` |
