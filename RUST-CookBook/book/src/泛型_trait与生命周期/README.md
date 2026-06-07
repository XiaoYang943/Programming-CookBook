# 泛型、trait 与生命周期

> 《Rust 程序设计语言》原文：[泛型、trait 与生命周期](https://doc.rust-lang.org/book/ch10-00-generics.html)。

> 《Rust Reference》：[泛型参数](https://doc.rust-lang.org/reference/items/generics.html)。

> 《Rust Reference》：[Trait](https://doc.rust-lang.org/reference/items/traits.html)。

> 《Rust Reference》：[实现](https://doc.rust-lang.org/reference/items/implementations.html)。

> 《Rust Reference》：[关联项](https://doc.rust-lang.org/reference/items/associated-items.html)。

> 《Rust Reference》：[Trait 与生命周期约束](https://doc.rust-lang.org/reference/trait-bounds.html)。

> 《Rust Reference》：[Trait Object](https://doc.rust-lang.org/reference/types/trait-object.html)。

> 《Rust Reference》：[`impl Trait`](https://doc.rust-lang.org/reference/types/impl-trait.html)。

> 《Rust Reference》：[类型参数](https://doc.rust-lang.org/reference/types/parameters.html)。

## 本章解决什么问题

泛型复用数据结构与算法，trait 定义类型能够提供的行为，生命周期描述引用之间的有效期关系。

## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 为函数、struct、enum 和方法添加类型参数 | 泛型数据类型 |
| 定义和实现共同行为 | 使用 trait 定义共同行为 |
| 描述返回引用与输入引用的关系 | 使用生命周期验证引用 |
| 选择静态分派或动态分派 | 理解泛型与 Trait Object 的权衡 |
| 使用关联类型、完全限定语法和 supertrait | 高级 trait |
| 理解生命周期子类型与 variance | 子类型与 Variance |

