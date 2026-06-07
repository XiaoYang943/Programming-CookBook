# 使用 Box&lt;T&gt; 指向堆上的数据

> 《Rust 程序设计语言》：[使用 `Box<T>` 指向堆上的数据](https://doc.rust-lang.org/book/ch15-01-box.html)。

## 使用 `Box<T>` 在堆上存储值

{{#playground demo/src/bin/box_堆上存储值.rs editable}}

## 使用 `Box<T>` 定义递归类型

递归类型需要通过 `Box<T>` 提供已知大小的间接层。

{{#playground demo/src/bin/box_定义递归类型.rs editable}}

