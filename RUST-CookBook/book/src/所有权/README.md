# 所有权

> 《Rust 程序设计语言》原文：[所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)。

> 《Rust Reference》：[内存分配与生命周期](https://doc.rust-lang.org/reference/memory-allocation-and-lifetime.html)。

> 《Rust Reference》：[指针类型](https://doc.rust-lang.org/reference/types/pointer.html)。

> 《Rust Reference》：[Slice 类型](https://doc.rust-lang.org/reference/types/slice.html)。

> 《Rust Reference》：[析构器](https://doc.rust-lang.org/reference/destructors.html)。

> 《Rust Reference》：[生命周期省略](https://doc.rust-lang.org/reference/lifetime-elision.html)。

## 所有权解决什么问题

Rust 使用所有权系统管理内存和其他资源，无需垃圾回收器，也不要求开发者手动释放内存。编译器会在编译期检查所有权与借用规则。

## 三条所有权规则

1. Rust 中每个值都有一个所有者。
2. 同一时刻，一个值只能有一个所有者。
3. 所有者离开作用域时，值会被丢弃。

## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 理解作用域、移动、`clone` 与 `Copy` | 什么是所有权？ |
| 在不转移所有权时访问或修改值 | 引用与借用 |
| 借用连续数据的一部分 | Slice 类型 |
| 选择引用、智能指针与原始指针 | 熟悉引用与指针类型 |
| 理解编译器如何验证借用规则 | 理解 Borrow Checker |
| 设计更灵活的函数参数 | 使用借用类型作为函数参数 |
| 从可变引用中安全取出值 | 使用 `mem::take` 和 `mem::replace` 转移可变引用中的所有权 |
| 避免使用无意义的 `clone` 绕过借用问题 | 不要为了满足 Borrow Checker 而 `clone` |

