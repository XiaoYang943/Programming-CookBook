# 使用类型系统表达数据结构

> 本文对应 Effective Rust **Item 1**：[Use the type system to express your data structures](https://effective-rust.com/use-types.html)。

## 使用 newtype 区分相同底层类型

**核心规则**

两个值即使都使用 `u64` 存储，也可能表达完全不同的业务含义。使用 Newtype 可以让编译器阻止误用。

{{#playground demo/src/bin/newtype_区分相同底层类型.rs editable}}

**错误示例**

不同 Newtype 不能被意外互换。

{{#playground snippets/newtype_不同语义不能混用.rs editable ignore mdbook-runnable}}

## 使用 `enum` 表达有限状态

**核心规则**

当数据只能处于有限状态之一时，使用 `enum` 将状态和对应数据绑定在一起。

{{#playground demo/src/bin/enum_让无效状态难以表示.rs editable}}

## 使用 `Option<T>` 表达值可能缺失

**核心规则**

使用 `Option<T>` 表达值存在或缺失，不使用 `-1`、空字符串等特殊值承担额外含义。

{{#playground demo/src/bin/option_表达值可能缺失.rs editable}}

## 建模检查表

- 不同语义是否错误地复用了同一种基础类型？
- 状态和仅在该状态下有效的数据是否被分开存储？
- 缺失值是否使用 `Option<T>` 表达？
- 无效状态能否在构造数据时就被类型系统阻止？
