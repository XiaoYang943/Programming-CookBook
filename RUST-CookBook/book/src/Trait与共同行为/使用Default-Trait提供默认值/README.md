# 使用 `Default` 提供默认值

> 本文整理自 Rust Design Patterns 的 Idiom：[The Default Trait](https://rust-unofficial.github.io/patterns/idioms/default.html)。

## 派生 `Default`

所有字段都实现 `Default` 时，可以直接派生默认实现。

{{#playground demo/src/bin/default_派生默认值.rs editable}}

## 手动实现 `Default`

默认值具有领域含义时，可以手动实现。

{{#playground demo/src/bin/default_手动实现领域默认值.rs editable}}

## 使用 struct 更新语法覆盖部分默认值

{{#playground demo/src/bin/default_结合struct更新语法.rs editable}}
