# 使用类型系统表达共同行为

> 本文对应 Effective Rust **Item 2**：[Use the type system to express common behavior](https://effective-rust.com/use-types-2.html)。

## 使用 trait 表达开放的共同行为

当未来可能增加新类型时，使用 trait 让类型独立实现共同协议。

{{#playground demo/src/bin/trait_表达开放共同行为.rs editable}}

## 使用 enum 表达封闭的类型集合

当所有可能类型已知并需要穷尽处理时，enum 更适合。

{{#playground demo/src/bin/enum_表达封闭类型集合.rs editable}}

## 使用 newtype 增加语义与约束

{{#playground demo/src/bin/newtype_表达独立语义.rs editable}}

## 选择类型表达方式

| 需求 | 选择 |
| --- | --- |
| 类型集合开放、行为一致 | trait |
| 类型集合封闭、需要穷尽匹配 | enum |
| 相同底层类型具有不同语义 | newtype |
