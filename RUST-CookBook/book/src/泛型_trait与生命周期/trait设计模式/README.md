# trait 设计模式

学会“定义一个 trait”之后，可以继续看 trait 如何设计成稳定、可扩展的库接口。

这一章继续参考 `geo`，但示例只保留 Rust 类型系统的关键骨架。

## default method 默认方法

真实工程参考：

- geo `Validation` trait：<https://github.com/georust/geo/blob/main/geo/src/algorithm/validation/mod.rs>

默认方法适合表达这种设计：

> 实现者只需要提供最核心的方法，trait 根据这个核心方法派生出其他通用方法。

这样可以减少重复实现，也能让 trait 的调用体验更完整。

{{#playground demo/src/bin/default_method_默认方法.rs editable}}

## Cow 生命周期

真实工程参考：

- geo `GeometryCow`：<https://github.com/georust/geo/blob/main/geo/src/geometry_cow.rs>

`Cow<'a, T>` 的全称是 clone-on-write。它常用于 API 同时接受“借用数据”和“拥有数据”的场景。

学习重点不是记住 `Cow` 的名字，而是理解它解决的问题：

- 调用方已经有引用时，不必强行 clone。
- 调用方愿意交出所有权时，也可以直接传入拥有值。
- API 内部用统一类型处理两种输入。

{{#playground demo/src/bin/cow_生命周期.rs editable}}

## GAT 泛型关联类型

真实工程参考：

- geo-traits `GeometryTrait`：<https://github.com/georust/geo/blob/main/geo-traits/src/geometry.rs>

`GeometryTrait` 里有大量类似这样的定义：

```rust
type PointType<'a>: PointTrait<T = Self::T>
where
    Self: 'a;
```

这就是 GAT：Generic Associated Types，泛型关联类型。

普通关联类型是：

```rust
type Output;
```

GAT 是：

```rust
type Output<'a>
where
    Self: 'a;
```

它适合表达：

> 从 `self` 里借出一部分数据时，返回类型和这次借用的生命周期有关。

下面的 demo 简化了 `geo-traits` 的 `GeometryTrait`。真实源码支持很多类型；示例只保留两个分支，让你看清 GAT、关联类型和枚举分发之间的关系。

{{#playground demo/src/bin/gat_泛型关联类型.rs editable}}
