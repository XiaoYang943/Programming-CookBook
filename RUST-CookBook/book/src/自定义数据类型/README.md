# 自定义数据类型

> 《Rust 程序设计语言》原文：[使用 struct 组织关联数据](https://doc.rust-lang.org/book/ch05-00-structs.html)。

> 《Rust 程序设计语言》原文：[enum 与模式匹配](https://doc.rust-lang.org/book/ch06-00-enums.html)。

> 《Rust Reference》：[Struct 声明](https://doc.rust-lang.org/reference/items/structs.html)。

> 《Rust Reference》：[Enum 声明](https://doc.rust-lang.org/reference/items/enumerations.html)。

> 《Rust Reference》：[Struct 表达式](https://doc.rust-lang.org/reference/expressions/struct-expr.html)。

> 《Rust Reference》：[Match 表达式](https://doc.rust-lang.org/reference/expressions/match-expr.html)。

## 自定义数据类型解决什么问题

自定义数据类型把领域规则编码到类型系统中。`struct` 将多个同时存在的字段组合为一个类型；`enum` 表达一个值只能处于若干种状态之一；模式匹配负责安全地拆解不同状态及其数据。

| 类型 | 表达能力 | 典型场景 |
| --- | --- | --- |
| `struct` | 多个字段同时存在 | 用户、配置、坐标、请求 |
| `enum` | 多个变体中只存在一个 | 状态、命令、可选值、错误 |

## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 定义字段并创建 struct 实例 | 定义与实例化 struct |
| 从重复参数重构出领域类型 | 使用 struct 的示例程序 |
| 使用 `impl` 定义方法与关联函数 | 方法 |
| 分步骤构造复杂类型 | 为复杂类型使用 Builder 模式 |
| 使用 `new` 提供惯用构造入口 | Constructor 构造函数惯例 |
| 定义 enum 变体及其关联数据 | 定义 enum |
| 使用 `match` 穷尽处理所有状态 | `match` 控制流结构 |
| 使用 `if let` 与 `let...else` 关注部分状态 | 使用 `if let` 和 `let...else` 简化控制流 |
