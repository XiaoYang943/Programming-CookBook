# 向闭包传递变量

> 本文整理自 Rust Design Patterns 的 Idiom：[Pass Variables to Closure](https://rust-unofficial.github.io/patterns/idioms/pass-var-to-closure.html)。

## 使用局部绑定传递环境值

闭包会捕获它实际使用的变量。先创建语义明确的局部绑定，可以控制捕获内容，并减少对外层结构的借用范围。

{{#playground demo/src/bin/binding_使用局部变量控制捕获.rs editable}}

## 在 `move` 前克隆共享值

多个 `move` 闭包都需要拥有同一份可克隆数据时，应在创建每个闭包前分别克隆，而不是试图多次移动原值。

{{#playground demo/src/bin/move_为多个闭包分别克隆.rs editable}}

## 使用 `Arc` 在闭包间共享所有权

线程闭包需要共享同一份只读数据时，可以克隆 `Arc`，让每个 `move` 闭包拥有一个引用计数句柄。

{{#playground demo/src/bin/arc_在线程闭包间共享值.rs editable}}

## 实践建议

- 只在闭包需要取得所有权时使用 `move`。
- 克隆尽量靠近闭包创建位置，让所有权转移清晰可见。
- 多线程共享可变状态时，结合 `Arc` 与同步原语，并缩小锁的作用域。
