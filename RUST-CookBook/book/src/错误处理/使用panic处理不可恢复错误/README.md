# 使用 panic! 处理不可恢复错误

> 《Rust 程序设计语言》原文：[使用 panic! 处理不可恢复错误](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)。

## 使用 `panic!` 终止不可恢复流程

`panic!` 会展开栈或直接终止进程，适合程序无法继续维持正确状态的场景。

{{#playground demo/src/bin/panic_处理不可恢复状态.rs editable should_panic}}

## 使用断言检查程序不变量

`assert!`、`assert_eq!` 和 `assert_ne!` 在条件不满足时触发 panic，适合验证程序内部不变量。

{{#playground demo/src/bin/assert_检查程序不变量.rs editable}}

