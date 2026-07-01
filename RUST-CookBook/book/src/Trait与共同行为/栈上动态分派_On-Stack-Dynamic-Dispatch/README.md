# 栈上动态分派

> 本文整理自 Rust Design Patterns 的 Idiom：[On-Stack Dynamic Dispatch](https://rust-unofficial.github.io/patterns/idioms/on-stack-dyn-dispatch.html)。

## 使用 `&dyn Trait` 借用栈上值

Trait Object 不一定需要 `Box<dyn Trait>`。引用可以指向栈上的不同具体类型并进行动态分派。

{{#playground demo/src/bin/dyn_trait_借用栈上不同类型.rs editable}}

## 选择借用或拥有 Trait Object

| 需求 | 选择 |
| --- | --- |
| 临时调用，值由其他位置拥有 | `&dyn Trait` |
| 集合需要拥有异构值 | `Box<dyn Trait>` |
