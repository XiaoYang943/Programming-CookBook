# 理解生命周期

> Effective Rust **Item 14**：[Understand lifetimes](https://effective-rust.com/lifetimes.html)。

## 生命周期描述引用关系

生命周期标注描述引用如何关联，不负责延长数据寿命。

{{#playground demo/src/bin/lifetime_描述引用关系.rs editable}}

## 让拥有类型替代复杂生命周期

如果数据需要独立长期保存，拥有 `String` 等值通常比保存引用更简单。

{{#playground demo/src/bin/owned_data_使用拥有类型简化生命周期.rs editable}}

## 生命周期设计原则

- 优先依赖生命周期省略规则。
- 只在需要表达多个引用关系时添加显式生命周期。
- 不要把 `'static` 当作解决生命周期错误的通用手段。
