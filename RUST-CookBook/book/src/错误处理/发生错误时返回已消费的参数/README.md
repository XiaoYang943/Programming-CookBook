# 发生错误时返回已消费的参数

> 本文整理自 Rust Design Patterns 的 Idiom：[Return consumed arg on error](https://rust-unofficial.github.io/patterns/idioms/return-consumed-arg-on-error.html)。

## 在 Error 中返还被消费值

函数按值接收参数且操作可能失败时，可以让 Error 携带输入值，使调用方恢复所有权。

{{#playground demo/src/bin/error_失败时返还被消费参数.rs editable}}

## 使用场景

- 资源注册、发送或写入失败后，调用方可能重试。
- 输入值复制成本高或无法 `clone`。
- 返还所有权比借用参数更符合 API 生命周期。
