# 使用 if let 和 let...else 简化控制流

> 《Rust 程序设计语言》原文：[使用 if let 和 let...else 简化控制流](https://doc.rust-lang.org/book/ch06-03-if-let.html)。

## 使用 `if let` 处理单个模式

只关心一个模式时，`if let` 比只包含一个有效分支的 `match` 更简洁。

{{#playground demo/src/bin/if_let_处理单个模式.rs editable}}

## 使用 `let...else` 提前处理失败分支

`let...else` 要求 `else` 分支发散，适合先验证模式并让成功路径保持平直。

{{#playground demo/src/bin/let_else_提前处理失败分支.rs editable}}

## 选择 `match`、`if let` 或 `let...else`

| 需求 | 选择 |
| --- | --- |
| 处理所有可能情况 | `match` |
| 只处理一个模式 | `if let` |
| 模式不匹配时提前退出 | `let...else` |

