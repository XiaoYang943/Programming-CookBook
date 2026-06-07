# 所有权

> 《Rust 程序设计语言》原文：[所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)。

> 《Rust Reference》：[内存分配与生命周期](https://doc.rust-lang.org/reference/memory-allocation-and-lifetime.html)。

> 《Rust Reference》：[指针类型](https://doc.rust-lang.org/reference/types/pointer.html)。

> 《Rust Reference》：[Slice 类型](https://doc.rust-lang.org/reference/types/slice.html)。

> 《Rust Reference》：[析构器](https://doc.rust-lang.org/reference/destructors.html)。

> 《Rust Reference》：[生命周期省略](https://doc.rust-lang.org/reference/lifetime-elision.html)。

> 《Rust 程序设计语言》原文：[什么是所有权](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)。
> 
## 内存释放的三种方式
1. 垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存(Java、Go)
2. 手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存(C++)
3. 通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查(Rust)

## 所有权规则
1. 每一个值都被一个变量所拥有
2. 一个值只能拥有一个所有者
3. 当所有者离开作用域范围时，这个值将被丢弃(drop)
4. 
## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 理解作用域、移动、`clone` 与 `Copy` | 什么是所有权 |
| 在不转移所有权时访问或修改值 | 引用与借用 |
| 借用连续数据的一部分 | Slice 类型 |
| 选择引用、智能指针与原始指针 | 熟悉引用与指针类型 |
| 理解编译器如何验证借用规则 | 理解 Borrow Checker |
| 从可变引用中安全取出值 | 使用 `mem::take` 和 `mem::replace` 转移可变引用中的所有权 |
| 避免使用无意义的 `clone` 绕过借用问题 | 不要为了满足 Borrow Checker 而 `clone` |

