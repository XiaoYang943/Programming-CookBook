# 高级函数与闭包

> 《Rust 程序设计语言》原文：[高级函数与闭包](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html)。

> 《Rust Reference》：[函数项类型](https://doc.rust-lang.org/reference/types/function-item.html)。

> 《Rust Reference》：[函数指针类型](https://doc.rust-lang.org/reference/types/function-pointer.html)。

> 《Rust Reference》：[调用表达式](https://doc.rust-lang.org/reference/expressions/call-expr.html)。

> 《Rust Reference》：[Return 表达式](https://doc.rust-lang.org/reference/expressions/return-expr.html)。

## 使用 `fn` 接收函数指针

函数项可以自动转换为函数指针。函数指针实现 `Fn`、`FnMut` 与 `FnOnce`，因此也能传给接收闭包的 API。

{{#playground demo/src/bin/fn_pointer_接收函数指针.rs editable}}

## 将 enum 构造函数作为函数传递

enum variant 构造函数和 tuple struct 构造函数都可以作为函数项传给 `map`。

{{#playground demo/src/bin/constructor_将构造函数传给map.rs editable}}

## 使用 `impl Fn` 返回单一闭包类型

每个闭包都有独有的匿名类型。函数只返回一种具体闭包时，可以使用 `impl Fn` 隐藏该类型。

{{#playground demo/src/bin/impl_fn_返回单一闭包类型.rs editable}}

## 使用 `Box<dyn Fn>` 返回不同闭包类型

不同分支返回不同闭包时，需要通过 Trait Object 擦除具体类型。

{{#playground demo/src/bin/box_dyn_fn_返回不同闭包类型.rs editable}}

## 选择函数指针或闭包

- 需要捕获环境时使用闭包。
- 只接受普通函数，或与要求函数指针的外部接口交互时使用 `fn`。
- API 通常优先接收 `Fn` 系列 trait bound，因为它同时兼容闭包与函数指针。

