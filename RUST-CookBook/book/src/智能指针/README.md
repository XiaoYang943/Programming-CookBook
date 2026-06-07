# 智能指针

> 《Rust 程序设计语言》原文：[智能指针](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)。

> 《Rust Reference》：[内部可变性](https://doc.rust-lang.org/reference/interior-mutability.html)。

> 《Rust Reference》：[析构器](https://doc.rust-lang.org/reference/destructors.html)。

## 智能指针是什么

智能指针是拥有数据并提供额外行为的数据结构，通常实现 `Deref` 和 `Drop`。

## 选择智能指针

| 需求 | 选择 |
| --- | --- |
| 堆上唯一所有权 | `Box<T>` |
| 单线程共享所有权 | `Rc<T>` |
| 运行时检查内部可变借用 | `RefCell<T>` |
| 自动清理资源 | `Drop` 与 RAII |

