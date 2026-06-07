# 使用 `Drop` 实现 RAII 模式

> 本文对应 Effective Rust **Item 11**：[Implement the Drop trait for RAII patterns](https://effective-rust.com/raii.html)。

## 使用资源守卫实现 RAII

资源获取与对象初始化绑定，资源释放与 `Drop` 绑定，使所有退出路径都能自动清理。

{{#playground demo/src/bin/raii_资源守卫自动清理.rs editable}}

## 理解字段的 `drop` 顺序

局部变量按声明的逆序释放，struct 字段按声明顺序释放。

{{#playground demo/src/bin/drop_order_观察释放顺序.rs editable}}
