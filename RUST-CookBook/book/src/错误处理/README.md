# 错误处理

> 《Rust 程序设计语言》：[错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)。

> 《Rust Reference》：[Panic](https://doc.rust-lang.org/reference/panic.html)。

> 《Rust Reference》：[发散](https://doc.rust-lang.org/reference/divergence.html)。

> 《Rust Reference》：[Never 类型](https://doc.rust-lang.org/reference/types/never.html)。

## 选择错误处理方式

| 情况 | 选择 |
| --- | --- |
| 调用方可以恢复或决定策略 | `Result<T, E>` |
| 值可能自然缺失 | `Option<T>` |
| 程序状态已损坏或违反不可恢复约束 | `panic!` |

## 错误处理原则

- 库优先返回错误，让调用方决定处理策略。
- 使用 `?` 传播当前层无法合理处理的错误。
- 为公共接口提供稳定、可理解的错误类型。

