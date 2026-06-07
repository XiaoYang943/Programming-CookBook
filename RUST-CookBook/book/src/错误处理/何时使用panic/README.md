# 何时使用 panic!

> 《Rust 程序设计语言》原文：[何时使用 panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)。

## 对可恢复错误返回 `Result`

输入无效、文件缺失、网络失败等情况可能由调用方处理，不应直接 panic。

{{#playground demo/src/bin/result_让调用方决定恢复策略.rs editable}}

## 对违反程序不变量的情况使用 `panic!`

示例代码、测试、原型和已经由前置检查保证安全的场景，可以使用 `panic!`、`unwrap` 或 `expect`。

## 判断问题

- 调用方能否采取合理恢复措施？
- 失败是否来自外部输入或环境？
- 继续执行是否会破坏程序正确性？

