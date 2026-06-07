# 优先选择 `Option` 和 `Result` 转换

> 本文对应 Effective Rust **Item 3**：[Prefer Option and Result transforms over explicit match expressions](https://effective-rust.com/transform.html)。

## 使用 `map` 转换成功值

{{#playground demo/src/bin/map_转换Option与Result成功值.rs editable}}

## 使用 `and_then` 串联可失败操作

{{#playground demo/src/bin/and_then_串联可失败操作.rs editable}}

## 使用 `or_else` 延迟提供替代方案

{{#playground demo/src/bin/or_else_提供替代方案.rs editable}}

## 何时使用 `match`

当不同分支需要执行明显不同的副作用、记录上下文或组合复杂模式时，显式 `match` 通常更清晰。
