# 熟悉引用与指针类型

> 本文对应 Effective Rust **Item 8**：[Familiarize yourself with reference and pointer types](https://effective-rust.com/references.html)。

## 使用 `&T` 与 `&mut T` 临时借用值

**核心规则**

引用不拥有数据。`&T` 用于共享读取，`&mut T` 用于独占修改，生命周期由 Borrow Checker 验证。

{{#playground demo/src/bin/reference_共享读取与独占修改.rs editable}}

## 使用 `Box<T>` 获得堆上唯一所有权

**核心规则**

`Box<T>` 在堆上存储值，并保持单一所有者。离开作用域时，Box 和其内容会一起释放。

{{#playground demo/src/bin/box_堆上唯一所有权.rs editable}}

## 使用 `Rc<T>` 共享单线程所有权

**核心规则**

`Rc<T>` 通过引用计数允许同一个值拥有多个所有者，仅适用于单线程。

{{#playground demo/src/bin/rc_共享单线程所有权.rs editable}}

## 使用 `Arc<T>` 共享多线程所有权

**核心规则**

`Arc<T>` 使用原子引用计数，可以在线程间共享所有权。共享可变状态通常还需要配合同步类型。

{{#playground demo/src/bin/arc_共享多线程所有权.rs editable}}

## 谨慎使用 `*const T` 与 `*mut T`

**核心规则**

原始指针不受普通借用规则保护，解引用需要 `unsafe`。它们主要用于 FFI、底层数据结构和性能敏感边界。

{{#playground demo/src/bin/raw_pointer_读取原始指针.rs editable}}

## 选择引用与指针类型

| 需求 | 选择 |
| --- | --- |
| 临时共享读取 | `&T` |
| 临时独占修改 | `&mut T` |
| 堆上唯一所有权 | `Box<T>` |
| 单线程共享所有权 | `Rc<T>` |
| 多线程共享所有权 | `Arc<T>` |
| 与底层接口交互且能维护安全约束 | 原始指针 |
