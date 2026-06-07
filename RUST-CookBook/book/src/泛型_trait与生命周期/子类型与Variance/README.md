# 子类型与 Variance

本节整理生命周期子类型关系以及泛型参数的协变、逆变与不变规则。

> 《Rust Reference》：[子类型与 Variance](https://doc.rust-lang.org/reference/subtyping.html)。

## 理解生命周期子类型

更长的生命周期可以在需要更短生命周期的位置使用。`'static` 比任意较短生命周期存活更久。

{{#playground demo/src/bin/lifetime_subtyping_长生命周期缩短使用.rs editable}}

## 理解 variance 影响类型转换

`&'a T` 对生命周期通常是协变的，`&'a mut T` 对被引用类型通常是不变的。variance 主要影响编译器判断泛型类型能否安全替换。

## 实践建议

- 日常代码优先依赖编译器推断。
- 编写包含生命周期、函数指针、内部可变性或 unsafe 的通用容器时，再深入分析 variance。

