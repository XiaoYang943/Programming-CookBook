# 理解泛型与 Trait Object 的权衡

> Effective Rust **Item 12**：[Understand the trade-offs between generics and trait objects](https://effective-rust.com/generics.html)。

## 使用泛型进行静态分派

泛型通过单态化生成具体代码，通常具有更好的内联机会，但可能增加编译时间和二进制体积。

{{#playground demo/src/bin/generic_静态分派.rs editable}}

## 使用 Trait Object 进行动态分派

`dyn Trait` 允许在运行时处理不同具体类型，适合异构集合和运行时可替换实现。

{{#playground demo/src/bin/trait_object_动态分派.rs editable}}

## 理解 dyn 兼容限制

不是所有 trait 都能构造 Trait Object，例如返回 `Self` 的方法通常不满足 dyn 兼容要求。

{{#playground snippets/trait_object_不满足dyn兼容要求.rs editable ignore mdbook-runnable}}

## 选择分派方式

| 需求 | 选择 |
| --- | --- |
| 性能优先、具体类型在编译期已知 | 泛型 |
| 异构集合、运行时替换实现 | Trait Object |
| 隐藏返回具体类型但保持静态分派 | `impl Trait` |
