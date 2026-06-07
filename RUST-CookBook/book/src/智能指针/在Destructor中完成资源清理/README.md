# 在析构器中完成资源清理

> 本文整理自 Rust Design Patterns 的 Idiom：[Finalisation in Destructors](https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html)。

## 在 `Drop` 中执行必需清理

{{#playground demo/src/bin/drop_执行必需清理.rs editable}}

## 不在 `Drop` 中依赖可恢复错误

`drop` 不能返回错误。需要让调用方处理失败时，应额外提供显式的 `close`、`flush` 或 `finish` 方法。

{{#playground demo/src/bin/finish_显式处理清理错误.rs editable}}
