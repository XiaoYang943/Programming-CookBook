# 使用 Box&lt;T&gt; 指向堆上的数据

> 《Rust 程序设计语言》：[使用 `Box<T>` 指向堆上的数据](https://doc.rust-lang.org/book/ch15-01-box.html)。

## 使用 `Box<T>` 在堆上存储值

{{#playground demo/src/bin/box_堆上存储值.rs editable}}

## 使用 `Box<T>` 定义递归类型

二叉树中的每个节点可能拥有左子树和右子树。下面这种直接递归的定义无法编译：

```rust,compile_fail
struct Node {
    value: i32,
    left: Option<Node>,
    right: Option<Node>,
}
```

为了计算一个 `Node` 所需的内存大小，编译器需要先知道其内部
`Option<Node>` 的大小；但计算 `Option<Node>` 的大小又需要知道
`Node` 的大小，形成了无法结束的递归。

`Box<Node>` 是一个大小固定的指针，实际的子节点存储在堆上。因此，
`Option<Box<Node>>` 为递归结构提供了已知大小的间接层：

```text
栈上的根节点
├── left:  指针 ──> 堆上的左子节点
└── right: 指针 ──> 堆上的右子节点
```

当子节点不存在时使用 `None`；存在时使用
`Some(Box::new(Node::new(value)))` 创建并拥有子节点。

{{#playground demo/src/bin/box_定义递归类型.rs editable}}
