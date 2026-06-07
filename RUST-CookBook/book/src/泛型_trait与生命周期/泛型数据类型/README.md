# 泛型数据类型

> 《Rust 程序设计语言》原文：[泛型数据类型](https://doc.rust-lang.org/book/ch10-01-syntax.html)。

## 使用泛型函数复用算法

类型参数写在函数名后的 `<T>` 中，trait bound 用于约束函数体可以使用的行为。

{{#playground demo/src/bin/generic_function_复用比较算法.rs editable}}

## 使用泛型 struct 存储不同类型

{{#playground demo/src/bin/generic_struct_存储泛型字段.rs editable}}

## 使用泛型 enum 表达通用状态

标准库中的 `Option<T>` 和 `Result<T, E>` 都是泛型 enum。

{{#playground demo/src/bin/generic_enum_表达通用状态.rs editable}}

## 为泛型类型实现方法

可以为所有 `T` 实现通用方法，也可以只为具体类型实现专用方法。

{{#playground demo/src/bin/generic_impl_实现通用与专用方法.rs editable}}

## 理解单态化

Rust 在编译期为使用到的具体类型生成代码，因此泛型通常不引入运行时分派成本。

