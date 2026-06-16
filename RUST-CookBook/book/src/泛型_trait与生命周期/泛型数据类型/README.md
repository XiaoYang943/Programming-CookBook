# 泛型数据类型

> 《Rust 程序设计语言》：[泛型数据类型](https://doc.rust-lang.org/book/ch10-01-syntax.html)。

## 使用泛型函数复用算法

类型参数写在函数名后的 `<T>` 中。同一份泛型函数可以处理多种具体类型，但泛型
参数本身不保证具备任何行为。

例如，冒泡排序算法需要使用 `>` 比较相邻元素。编译器只有在确认 `T` 实现了
`PartialOrd` 后，才允许函数体比较两个 `T`：

```rust
fn bubble_sort<T>(values: &mut [T])
where
    T: PartialOrd,
{
    todo!()
}
```

`T: PartialOrd` 是 trait bound，表示调用方传入的元素类型必须实现
`PartialOrd`。这个约束既限制了可以传入的类型，也明确了函数体可以对 `T`
使用的行为。

如果移除这个 trait bound，`value > current` 将无法编译，因为编译器不知道任意
类型 `T` 是否支持 `>` 比较。

### 使用 `PartialOrd` 进行部分排序

`PartialOrd` 表示两个值通常可以比较大小，其核心方法 `partial_cmp` 返回
`Option<Ordering>`：

- `Some(Ordering::Less)`：左值小于右值。
- `Some(Ordering::Equal)`：两个值相等。
- `Some(Ordering::Greater)`：左值大于右值。
- `None`：两个值无法确定顺序。

整数、字符以及许多业务类型都实现或可以派生 `PartialOrd`。浮点数也实现了
`PartialOrd`，但 `NaN` 与任何浮点数都无法确定顺序：

```rust
assert_eq!(f64::NAN.partial_cmp(&1.0), None);
```

因此，使用 `PartialOrd` 的排序算法需要明确如何处理无法比较的值。示例中的
`bubble_sort` 使用 `>`，无法为包含 `NaN` 的浮点数提供可靠排序结果。对浮点数
排序有严格要求时，应先拒绝 `NaN`，或使用能够提供全序语义的包装类型。

如果业务要求任意两个值都必须存在确定顺序，应使用更严格的 `Ord`。整数实现了
`Ord`，而原生浮点数因为存在 `NaN`，没有实现 `Ord`。

示例中的 `bubble_sort` 不关心元素是整数、字符还是字符串。它只要求元素支持
`PartialOrd`，因此同一套排序算法可以复用于所有满足约束的类型。

排序时使用 `values.swap` 交换切片中的元素，不需要复制或克隆元素，因此不需要
额外增加 `Copy` 或 `Clone` trait bound。

{{#playground demo/src/bin/generic_function_复用比较算法.rs editable}}

## 使用泛型 struct 存储不同类型

{{#playground demo/src/bin/generic_struct_存储泛型字段.rs editable}}

## 使用泛型 enum 表达通用状态

标准库中的 `Option<T>` 和 `Result<T, E>` 都是泛型 enum。

{{#playground demo/src/bin/generic_enum_表达通用状态.rs editable}}

## 为泛型类型实现方法

可以为所有 `T` 实现通用方法，也可以只为具体类型实现专用方法。

{{#playground demo/src/bin/generic_impl_实现通用与专用方法.rs editable}}

## 理解单态化

Rust 在编译期为使用到的具体类型生成代码，因此泛型通常不引入运行时分派成本。
