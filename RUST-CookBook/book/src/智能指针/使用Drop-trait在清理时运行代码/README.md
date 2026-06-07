# 使用 Drop trait 在清理时运行代码

> 《Rust 程序设计语言》原文：[使用 Drop trait 在清理时运行代码](https://doc.rust-lang.org/book/ch15-03-drop.html)。

## 实现 `Drop` 自动执行清理

{{#playground demo/src/bin/drop_作用域结束自动清理.rs editable}}

## 使用 `drop` 提前释放值

不能直接调用 `Drop::drop`；需要提前释放时使用 `std::mem::drop`。

{{#playground demo/src/bin/drop_提前释放资源.rs editable}}

