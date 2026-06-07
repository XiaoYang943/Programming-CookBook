
> 《Rust 程序设计语言》：[引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
> 《Rust 设计模式》：[Use borrowed types for arguments](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)
> 《Effective Rust Item 8》：[Familiarize yourself with reference and pointer types](https://effective-rust.com/references.html)。

## 值的移动与复制

### 使用 `clone` 深拷贝

需要两个彼此独立的拥有者时，可以调用 `clone` 复制数据及其堆资源

{{#playground demo/src/bin/clone_深度复制数据.rs editable}}

### 使用 `Copy` 浅拷贝

浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在

{{#playground demo/src/bin/copy_复制栈上值.rs editable}}

- 可以浅拷贝的数据类型
  - 所有整数类型，比如 u32 
  - 布尔类型，bool，它的值是 true 和 false 
  - 所有浮点数类型，比如 f64 
  - 字符类型，char 
  - 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是 
  - 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意：可变引用 &mut T 是不可以 Copy的

## 值的所有权转移

### 移动赋值

将拥有堆资源的值赋给另一个变量时，所有权通常会移动到新变量，旧变量随后失效

{{#playground demo/src/bin/move_转移所有权.rs editable}}

### 参数传递与返回值

按值传参会移动或复制参数；返回值可以将所有权交给调用者

只需要临时访问值时，应使用引用

{{#playground demo/src/bin/function_通过参数和返回值转移所有权.rs editable}}

## 值的引用与借用

### 共享不可变引用 `&T`

共享不可变引用(shared_reference)，也叫借用(borrowing)

{{#playground demo/src/bin/shared_reference_共享不可变引用.rs editable}}

### 独占可变引用 `&mut T`

可变引用允许修改借用的值

创建可变引用时，原绑定必须使用 `mut` 声明

{{#playground demo/src/bin/mutable_reference_独占可变引用.rs editable}}

### 避免同时存在冲突借用

同一时刻，可以存在多个不可变引用，或者一个可变引用；不能同时存在会互相冲突的借用

{{#playground snippets/borrow_conflict_可变与不可变引用冲突.rs editable ignore mdbook-runnable}}

### 避免返回悬垂引用

引用不能比它指向的数据存活更久

Rust 会阻止返回指向局部变量的引用

{{#playground snippets/dangling_reference_不能返回局部引用.rs editable ignore mdbook-runnable}}



### 使用 `&str` 代替 `&String`

- &String 表示： 借用一个必须由 String 管理的字符串
- &str 表示： 借用任意一段有效的字符串(更通用)

{{#playground demo/src/bin/str_使用借用字符串参数.rs editable}}

### 使用 `&[T]` 代替 `&Vec<T>`

- `&Vec<T>` 表示：借用一个必须由 `Vec<T>` 管理的集合

- `&[T]` 表示：借用任意一段连续的 `T`（更通用）


只需要读取连续元素时，参数使用 `&[T]`它可以接收数组、`Vec<T>` 及其局部 Slice

{{#playground demo/src/bin/slice_使用借用序列参数.rs editable}}

### 使用 `&mut [T]` 修改连续元素

- 只修改固定范围内的已有元素，使用 `&mut [T]`，适用范围更广
- 需要添加、删除元素或调整容量，使用 `&mut Vec<T>`，获得容器管理能力

{{#playground demo/src/bin/mutable_slice_修改连续元素.rs editable}}

### 谨慎使用 `*const T` 与 `*mut T`

原始指针不受普通借用规则保护，解引用需要 `unsafe`。它们主要用于 FFI、底层数据结构和性能敏感边界

{{#playground demo/src/bin/raw_pointer_读取原始指针.rs editable}}

## 智能指针与所有权

### 使用 `Box<T>` 获得堆上唯一所有权

`Box<T>` 在堆上存储值，并保持单一所有者。离开作用域时，Box 和其内容会一起释放。

{{#playground demo/src/bin/box_堆上唯一所有权.rs editable}}

### 使用 `Rc<T>` 共享单线程所有权

`Rc<T>` 通过引用计数允许同一个值拥有多个所有者，仅适用于单线程。

{{#playground demo/src/bin/rc_共享单线程所有权.rs editable}}

### 使用 `Arc<T>` 共享多线程所有权

`Arc<T>` 使用原子引用计数，可以在线程间共享所有权。共享可变状态通常还需要配合同步类型。

{{#playground demo/src/bin/arc_共享多线程所有权.rs editable}}
