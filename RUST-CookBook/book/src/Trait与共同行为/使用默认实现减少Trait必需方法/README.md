# 使用默认实现减少 trait 必需方法

> Effective Rust **Item 13**：[Use default implementations to minimize required trait methods](https://effective-rust.com/default-impl.html)。

## 只要求实现最小核心方法

trait 应将无法推导的行为设为必需方法，其余行为通过默认实现组合得到。

{{#playground demo/src/bin/default_method_通过核心方法提供默认实现.rs editable}}

## 覆盖默认实现

实现类型可以覆盖默认方法，但仍应遵守 trait 的语义契约。

{{#playground demo/src/bin/default_method_覆盖默认实现.rs editable}}

## trait 设计原则

- 减少实现者必须编写的方法。
- 默认方法基于必需方法构建。
- 避免默认方法之间形成难以理解的调用关系。
