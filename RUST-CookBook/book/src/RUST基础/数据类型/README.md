# 数据类型

> 《Rust 程序设计语言》：[数据类型](https://doc.rust-lang.org/book/ch03-02-data-types.html)。

> 《Rust Reference》：[基本类型](https://doc.rust-lang.org/reference/types.html)。

> 《Rust Reference》：[布尔类型](https://doc.rust-lang.org/reference/types/boolean.html)。

> 《Rust Reference》：[数值类型](https://doc.rust-lang.org/reference/types/numeric.html)。

> 《Rust Reference》：[字符类型](https://doc.rust-lang.org/reference/types/char.html)。

> 《Rust Reference》：[字符串 Slice 类型](https://doc.rust-lang.org/reference/types/str.html)。

> 《Rust Reference》：[Tuple 类型](https://doc.rust-lang.org/reference/types/tuple.html)。

> 《Rust Reference》：[数组类型](https://doc.rust-lang.org/reference/types/array.html)。

> 《Rust Reference》：[字面量表达式](https://doc.rust-lang.org/reference/expressions/literal-expr.html)。

> 《Rust Reference》：[运算符表达式](https://doc.rust-lang.org/reference/expressions/operator-expr.html)。

> 《Rust Reference》：[数组表达式](https://doc.rust-lang.org/reference/expressions/array-expr.html)。

> 《Rust Reference》：[Tuple 表达式](https://doc.rust-lang.org/reference/expressions/tuple-expr.html)。

> 《Rust Reference》：[索引表达式](https://doc.rust-lang.org/reference/expressions/array-expr.html#index-expressions)。

> 《Rust Reference》：[Tuple 索引表达式](https://doc.rust-lang.org/reference/expressions/tuple-expr.html#tuple-indexing-expressions)。

## 使用类型标注消除类型推断歧义



Rust 是静态类型语言。编译器通常可以推断类型；存在多种可能类型时，需要显式标注。

{{#playground demo/src/bin/type_annotation_使用类型标注.rs editable}}

**语法限制**

无法推断目标类型时，编译器会要求补充类型标注。

{{#playground snippets/type_inference_无法推断parse目标类型.rs editable ignore mdbook-runnable}}

## 使用标量类型表示单个值



整数、浮点数、`bool` 和 `char` 都是标量类型。整数类型需要根据取值范围、符号和平台相关性选择。

{{#playground demo/src/bin/scalar_types_标量类型.rs editable}}

**实践建议**

- 数量和索引通常优先使用语义匹配的整数类型。
- 不要依赖整数溢出；需要明确行为时使用 `checked_*`、`wrapping_*` 或 `saturating_*`。
- `char` 表示 Unicode 标量值，不等同于单字节字符。

## 使用元组组合不同类型的值



Tuple 长度固定，可以组合不同类型，并通过解构或 `.索引` 读取元素。

{{#playground demo/src/bin/tuple_组合不同类型.rs editable}}

## 使用数组存储固定长度的同类型值



Array 的元素类型相同、长度固定，并存储在栈上。数组类型包含长度，例如 `[i32; 3]`。

{{#playground demo/src/bin/array_固定长度同类型.rs editable}}

**语法限制**

运行时索引越界会触发 panic。

{{#playground demo/src/bin/array_index_数组越界panic.rs editable should_panic}}

## 选择元组或数组

| 需求 | 选择 |
| --- | --- |
| 组合含义不同、类型可能不同的固定字段 | Tuple |
| 存储类型相同、长度固定的一组值 | Array |
| 长度需要动态变化 | 后续使用 `Vec<T>` |
