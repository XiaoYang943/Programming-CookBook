# 对 `Option` 进行迭代

> 本文整理自 Rust Design Patterns 的 Idiom：[Iterating over an Option](https://rust-unofficial.github.io/patterns/idioms/option-iter.html)。

## 使用 `iter` 将 `Option` 视为零个或一个元素

`Option::iter` 产生借用项：`Some` 产生一个元素，`None` 不产生元素。

{{#playground demo/src/bin/option_iter_借用零个或一个元素.rs editable}}

## 使用 `into_iter` 将可选值接入 Iterator 链

`Option<T>` 实现了 `IntoIterator`，适合把可选前缀、后缀或额外项接入现有序列。

{{#playground demo/src/bin/option_into_iter_连接可选元素.rs editable}}

## 使用 `flatten` 忽略 `None`

对 `Iterator<Item = Option<T>>` 调用 `flatten`，可以保留所有 `Some` 中的值并忽略 `None`。

{{#playground demo/src/bin/flatten_忽略None.rs editable}}

## 选择原则

- 只转换单个可选值时，优先使用 `Option::map`、`and_then` 等方法。
- 需要与其他 Iterator 统一组合时，再使用 `iter`、`into_iter` 或 `flatten`。
