# Rc&lt;T&gt;：引用计数智能指针

> 《Rust 程序设计语言》原文：[`Rc<T>`：引用计数智能指针](https://doc.rust-lang.org/book/ch15-04-rc.html)。

## 使用 `Rc<T>` 共享单线程所有权

`Rc::clone` 增加引用计数，不会深度复制内部数据。

{{#playground demo/src/bin/rc_共享所有权与引用计数.rs editable}}

## `Rc<T>` 使用限制

`Rc<T>` 不支持多线程共享，也不直接允许修改内部值。

