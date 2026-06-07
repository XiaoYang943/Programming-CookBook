# 使用生命周期验证引用

> 《Rust 程序设计语言》原文：[使用生命周期验证引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)。

## 使用生命周期标注关联输入与输出引用

生命周期标注不延长引用寿命，只描述多个引用之间的有效期关系。

{{#playground demo/src/bin/lifetime_关联输入与输出引用.rs editable}}

## 避免返回悬垂引用

{{#playground snippets/lifetime_不能返回局部变量引用.rs editable ignore mdbook-runnable}}

## 在 struct 中存储引用

包含引用的 struct 必须标注该引用的生命周期。

{{#playground demo/src/bin/lifetime_struct_在结构体中存储引用.rs editable}}

## 使用生命周期省略规则

常见函数签名中的生命周期可以由编译器根据省略规则推断。

{{#playground demo/src/bin/lifetime_elision_省略常见生命周期.rs editable}}

## 使用 `'static` 表达程序全程有效引用

字符串字面量具有 `'static` 生命周期；不要为了满足编译器而随意要求调用方提供 `'static` 引用。

{{#playground demo/src/bin/static_lifetime_字符串字面量.rs editable}}

