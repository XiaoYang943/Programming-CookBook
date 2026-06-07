# 不要为了满足 Borrow Checker 而 `clone`

> 本文整理自 Rust Design Patterns 的 Anti-pattern：[Clone to satisfy the borrow checker](https://rust-unofficial.github.io/patterns/anti_patterns/borrow_clone.html)。

## 缩小借用范围代替 `clone`



如果 `clone` 只是为了绕过借用冲突，先尝试缩短引用的存活时间，让后续修改发生在借用结束之后。

{{#playground demo/src/bin/borrow_scope_缩小借用范围代替clone.rs editable}}

## 拆分字段代替复制整个值



需要同时操作不同数据时，可以拆分结构体字段或调整数据结构，而不是复制整个对象。

{{#playground demo/src/bin/split_fields_拆分字段代替clone.rs editable}}

## 在确实需要独立所有权时使用 `clone`



`clone` 并非错误。当调用方和被调用方都需要独立拥有数据，或者数据必须跨线程、缓存或长期保存时，复制可能是正确选择。

{{#playground demo/src/bin/clone_需要独立所有权时复制.rs editable}}

## `clone` 检查表

- 复制是否属于业务语义，而不是只为消除编译错误？
- 能否通过缩小借用范围解决问题？
- 能否借用字段、拆分字段或调整函数参数？
- 数据复制成本是否清晰并可接受？
