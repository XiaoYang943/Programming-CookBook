# 使用 Result 处理可恢复错误

> 《Rust 程序设计语言》原文：[使用 Result 处理可恢复错误](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)。

## 使用 `Result<T, E>` 表达成功或失败

{{#playground demo/src/bin/result_表达可恢复错误.rs editable}}

## 使用 `match` 分别处理成功与错误

{{#playground demo/src/bin/result_match_处理成功与错误.rs editable}}

## 使用 `?` 传播错误

`?` 在成功时取出值，在失败时提前返回可转换的错误。

{{#playground demo/src/bin/question_mark_传播错误.rs editable}}

## 使用 `unwrap` 与 `expect` 表达已验证前提

`unwrap` 和 `expect` 会在错误时 panic。仅在失败确实代表程序错误时使用，优先让 `expect` 信息说明前提。

{{#playground demo/src/bin/expect_表达已验证前提.rs editable}}

