# 使用 Iterator 处理一系列项

> 《Rust 程序设计语言》：[使用 Iterator 处理一系列项](https://doc.rust-lang.org/book/ch13-02-iterators.html)。

> 《Rust Reference》：[Iterator 与循环](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops)。

## 使用 `next` 逐项推进 Iterator

Iterator 是惰性的：只有调用 `next` 或消费适配器时才会执行。`next` 返回 `Option<Self::Item>`，迭代结束后返回 `None`。

{{#playground demo/src/bin/next_逐项推进Iterator.rs editable}}

## 使用 `iter`、`iter_mut` 与 `into_iter` 选择所有权

| 方法 | 产生的典型项 | 对原集合的影响 |
| --- | --- | --- |
| `iter` | `&T` | 只读借用 |
| `iter_mut` | `&mut T` | 可变借用 |
| `into_iter` | `T` | 消费集合 |

{{#playground demo/src/bin/iter_选择借用与所有权.rs editable}}

## 使用适配器组合惰性转换

`map`、`filter` 等适配器只构造新的 Iterator。调用 `collect`、`sum`、`fold` 等消费适配器后，转换才真正运行。

{{#playground demo/src/bin/map_filter_collect_组合惰性转换.rs editable}}

## 使用消费适配器生成结果

消费适配器取得 Iterator 的所有权并推进迭代，常见方法包括 `sum`、`fold`、`find`、`any` 与 `collect`。

{{#playground demo/src/bin/consumers_消费Iterator生成结果.rs editable}}

## 实现 `Iterator` 创建自定义序列

实现 `Iterator` 只要求定义关联类型 `Item` 和 `next` 方法，其余适配器会自动可用。

{{#playground demo/src/bin/iterator_实现自定义计数器.rs editable}}

