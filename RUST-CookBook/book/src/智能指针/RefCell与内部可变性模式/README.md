# RefCell&lt;T&gt; 与内部可变性模式

> 《Rust 程序设计语言》：[`RefCell<T>` 与内部可变性模式](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)。

## 使用 `RefCell<T>` 在运行时检查借用

`RefCell<T>` 允许通过不可变绑定修改内部值，借用规则从编译期推迟到运行时检查。

{{#playground demo/src/bin/refcell_内部可变性.rs editable}}

## 避免运行时借用冲突

同时持有冲突的 `borrow` 与 `borrow_mut` 会触发 panic。

{{#playground demo/src/bin/refcell_借用冲突panic.rs editable should_panic}}

## 组合 `Rc<T>` 与 `RefCell<T>`

该组合可以表达单线程共享可变所有权，但需要谨慎控制复杂度。

{{#playground demo/src/bin/rc_refcell_共享可变状态.rs editable}}

