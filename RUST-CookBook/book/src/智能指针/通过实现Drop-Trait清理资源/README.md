# 通过实现Drop-Trait清理资源

> 《Rust 程序设计语言》：[通过实现Drop-Trait清理资源](https://doc.rust-lang.org/book/ch15-03-drop.html)。
> 《Rust 设计模式》：[Finalisation in Destructors](https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html)。

- Rust 的 `Drop` 与 C++ 析构函数作用相近：值离开作用域时，自动运行资源清理逻辑。这种把资源生命周期绑定到对象生命周期的方式称为 RAII。

- 普通数据会被 Rust 自动销毁，但 Rust 不知道某个业务资源应该如何清理
  - 例如，数据库连接使用结束后可能需要归还连接池，并发执行单元使用结束后需要归还线程池
  - 类型可以通过实现 `Drop` 定义这些额外的清理行为。

## 数据库连接自动归还连接池

`ConnectionPool::acquire` 返回一个 `PooledConnection`。它拥有取出的数据库连接，并在离开作用域时通过 `Drop` 自动归还连接。

即使函数中途 `return` 或发生 panic，栈展开时仍会执行 `Drop`，因此调用方不容易忘记归还连接。

{{#playground demo/src/bin/drop_作用域结束自动清理.rs editable}}

## 提前归还线程池资源

通常只需等待值离开作用域，不需要手动释放。但稀缺资源使用完毕后，继续持有它会阻塞其他任务，此时可以使用 `std::mem::drop` 提前销毁资源守卫。

不能直接调用 `Drop::drop`；需要提前释放时使用 `std::mem::drop`。

{{#playground demo/src/bin/drop_提前释放资源.rs editable}}

## 使用注意

- `Drop::drop` 不能返回错误。需要处理清理失败时，应额外提供显式的 `close`、`flush` 或 `finish` 方法。
- 不要直接调用值的 `Drop::drop` 方法，否则可能导致重复清理。提前销毁应使用 `std::mem::drop(value)`。
- 标准库中的 `Box<T>`、`Vec<T>`、`File`、`MutexGuard<T>` 等类型都依赖 `Drop` 自动释放资源。
