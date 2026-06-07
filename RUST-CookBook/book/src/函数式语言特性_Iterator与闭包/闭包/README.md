# 闭包

> 《Rust 程序设计语言》原文：[闭包](https://doc.rust-lang.org/book/ch13-01-closures.html)。

> 《Rust Reference》：[闭包表达式](https://doc.rust-lang.org/reference/expressions/closure-expr.html)。

> 《Rust Reference》：[闭包类型](https://doc.rust-lang.org/reference/types/closure.html)。

## 使用闭包保存行为

闭包使用 `|参数| 表达式` 定义，可以从调用位置推断参数与返回值类型。

{{#playground demo/src/bin/closure_定义并调用闭包.rs editable}}

## 捕获不可变引用与可变引用

编译器会根据闭包体的实际需要，选择不可变借用、可变借用或取得所有权。

{{#playground demo/src/bin/closure_捕获不可变与可变引用.rs editable}}

## 使用 `move` 取得环境值的所有权

`move` 强制闭包取得所使用环境值的所有权，常用于线程或需要让闭包离开当前作用域的场景。

{{#playground demo/src/bin/move_取得环境值所有权.rs editable}}

## 使用 `Fn`、`FnMut` 与 `FnOnce` 约束闭包

| trait | 闭包对捕获值的使用方式 | 调用次数 |
| --- | --- | --- |
| `Fn` | 只读借用 | 可多次 |
| `FnMut` | 可变借用 | 可多次 |
| `FnOnce` | 可能消费捕获值 | 至少可调用一次 |

每个闭包至少实现 `FnOnce`；没有消费捕获值时，还可能实现 `FnMut` 或 `Fn`。

{{#playground demo/src/bin/fn_traits_约束闭包行为.rs editable}}

## 理解闭包类型推断

闭包第一次调用后，参数与返回值的具体类型就会固定，不能再用另一种参数类型调用同一个闭包。

{{#playground snippets/closure_参数类型推断后固定.rs editable ignore mdbook-runnable}}

