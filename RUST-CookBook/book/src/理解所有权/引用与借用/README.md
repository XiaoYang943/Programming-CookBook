# 引用与借用

> 《Rust 程序设计语言》原文：[引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)。

## 使用 `&T` 共享读取值

**核心规则**

引用允许在不取得所有权的情况下访问值。创建引用称为借用，引用离开作用域不会释放被借用的值。

{{#playground demo/src/bin/shared_reference_共享不可变引用.rs editable}}

## 使用 `&mut T` 独占修改值

**核心规则**

可变引用允许修改借用的值。创建可变引用时，原绑定必须使用 `mut` 声明。

{{#playground demo/src/bin/mutable_reference_独占可变引用.rs editable}}

## 避免同时存在冲突借用

**核心规则**

同一时刻，可以存在多个不可变引用，或者一个可变引用；不能同时存在会互相冲突的借用。

{{#playground snippets/borrow_conflict_可变与不可变引用冲突.rs editable ignore mdbook-runnable}}

## 避免返回悬垂引用

**核心规则**

引用不能比它指向的数据存活更久。Rust 会阻止返回指向局部变量的引用。

{{#playground snippets/dangling_reference_不能返回局部引用.rs editable ignore mdbook-runnable}}

## 借用检查表

- 只读访问使用 `&T`。
- 需要修改使用 `&mut T`。
- 缩小引用的使用范围，避免不必要的借用冲突。
- 需要长期保存数据时，考虑取得所有权，而不是保存短期引用。
