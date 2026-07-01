# Trait 与共同行为

> 《Rust 程序设计语言》：[Trait：定义共同行为](https://doc.rust-lang.org/book/ch10-02-traits.html)。

trait 用来描述类型能够提供的行为。它既能支持静态分派的泛型约束，也能通过 trait object 表达运行时多态。

## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 定义和实现共同行为 | 使用 trait 定义共同行为 |
| 使用 trait object 抽象运行时多态 | 使用 trait object 抽象共同行为 |
| 熟悉标准库常见 trait | 熟悉标准Trait |
| 用默认方法降低实现成本 | 使用默认实现减少Trait必需方法 |
| 使用 Default 表达默认值 | 使用Default Trait提供默认值 |
| 比较泛型与 trait object 的权衡 | 理解泛型与Trait Object的权衡 |
| 使用关联类型、完全限定语法和 supertrait | 高级 trait |