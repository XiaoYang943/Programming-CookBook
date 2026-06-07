# 优先考虑 Iterator 转换而非显式循环

> Effective Rust **Item 9**：[Consider iterator transforms instead of explicit loops](https://effective-rust.com/iterators.html)。

## 使用 Iterator 表达转换意图

Iterator 链把筛选、转换与聚合分别交给标准适配器，通常比手动维护结果集合和控制状态更直接。

{{#playground demo/src/bin/filter_map_同时筛选并转换.rs editable}}

## 使用 `collect` 汇集可失败转换

当 Iterator 的项为 `Result<T, E>` 时，`collect::<Result<Vec<_>, _>>()` 会在全部成功时得到集合，并在首个错误处提前返回。

{{#playground demo/src/bin/collect_汇集Result.rs editable}}

## 使用 `fold` 累积自定义状态

简单求和优先使用 `sum`；确实需要同时维护多个状态时，再使用 `fold`。

{{#playground demo/src/bin/fold_累积多个状态.rs editable}}

## 选择原则

- 优先组合 `map`、`filter`、`filter_map`、`flat_map` 与消费适配器。
- Iterator 链表达不清晰时，拆成命名步骤或改用循环。
- 不为追求链式写法而隐藏副作用和复杂控制流。
