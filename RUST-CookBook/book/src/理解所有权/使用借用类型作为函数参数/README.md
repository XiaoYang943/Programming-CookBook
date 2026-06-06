# 使用借用类型作为函数参数

> 本文整理自 Rust Design Patterns 的 Idiom：[Use borrowed types for arguments](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)。

归入“理解所有权”，因为函数参数选择借用类型可以减少不必要的所有权转移，并提升API灵活性。

## 待补充

- `&str`与`&String`
- `&[T]`与`&Vec<T>`
- 配套Demo与实践检查表
