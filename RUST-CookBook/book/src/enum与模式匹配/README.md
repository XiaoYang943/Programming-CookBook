# enum 与模式匹配

> 《Rust 程序设计语言》原文：[enum 与模式匹配](https://doc.rust-lang.org/book/ch06-00-enums.html)。

> 《Rust Reference》：[Enum 声明](https://doc.rust-lang.org/reference/items/enumerations.html)。

> 《Rust Reference》：[Enum 类型](https://doc.rust-lang.org/reference/types/enum.html)。

> 《Rust Reference》：[Match 表达式](https://doc.rust-lang.org/reference/expressions/match-expr.html)。

## enum 与模式匹配解决什么问题

`enum` 表达“一个值只能处于若干种状态之一”，模式匹配则安全地拆解状态及其携带的数据。

## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 定义状态及其关联数据 | 定义 enum |
| 穷尽处理所有状态 | `match` 控制流结构 |
| 简化只关注少数状态的控制流 | 使用 `if let` 和 `let...else` 简化控制流 |

