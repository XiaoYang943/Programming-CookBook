# 智能指针

> 《Rust 程序设计语言》：[智能指针](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

> 《Rust Reference》：[内部可变性](https://doc.rust-lang.org/reference/interior-mutability.html)

> 《Rust Reference》：[析构器](https://doc.rust-lang.org/reference/destructors.html)

> Effective Rust **Item 11**：[Implement the Drop trait for RAII patterns](https://effective-rust.com/raii.html)。

## 智能指针是什么

智能指针是拥有数据并提供额外行为的数据结构，通常实现 `Deref` 和 `Drop`

### Deref
- 作用：如何访问内部值

### Drop
- 作用：如何销毁当前值


