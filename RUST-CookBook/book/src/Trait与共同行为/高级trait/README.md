# 高级 trait

> 《Rust 程序设计语言》：[高级 trait](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html)。

## 使用关联类型定义 trait 占位类型

关联类型让每个实现为 trait 选择一个确定的相关类型。

{{#playground demo/src/bin/associated_type_定义关联类型.rs editable}}

## 使用默认泛型参数支持运算符重载

{{#playground demo/src/bin/add_trait_使用默认泛型参数.rs editable}}

## 使用完全限定语法消除方法歧义

{{#playground demo/src/bin/fully_qualified_消除方法歧义.rs editable}}

## 使用 supertrait 声明依赖行为

{{#playground demo/src/bin/supertrait_声明依赖行为.rs editable}}

## 使用 newtype 绕过孤儿规则

{{#playground demo/src/bin/newtype_为外部类型实现外部trait.rs editable}}

