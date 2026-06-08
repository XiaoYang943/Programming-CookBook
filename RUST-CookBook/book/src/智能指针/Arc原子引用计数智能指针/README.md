# `Arc<T>`：原子引用计数智能指针

## `Arc<T>`
- Arc(Atomic Rc，原子化的引用计数只能指针)

## 使用 `Arc<T>` 在多线程编程中共享所有权

- `Arc<T>` 是线程安全版本的 `Rc<T>`
- 
- 当多个线程需要共同拥有同一份数据时，可以使用 `Arc<T>` 共享所有权

- `Arc<T>` 只解决“多个线程都能拥有同一份数据”的问题，不会自动让内部数据变得可修改需要并发读写时，通常会把它和锁一起使用：

```rust
Arc<RwLock<HashMap<K, V>>>
```

其中：

- `Arc<T>`：让多个线程都能持有同一个缓存
- `RwLock<T>`：允许多个读者同时读取；写入时需要独占访问
- `HashMap<K, V>`：保存真正的业务数据

{{#playground demo/src/bin/arc_rwlock_hashmap_共享缓存.rs editable}}
