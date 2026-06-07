# 理解 Borrow Checker

> 本文对应 Effective Rust **Item 15**：[Understand the borrow checker](https://effective-rust.com/borrows.html)。

## 使用 Borrow Checker 阻止悬垂引用



Borrow Checker 在编译期验证引用始终有效，并阻止数据竞争、悬垂引用和无效别名。

{{#playground snippets/lifetime_引用不能比数据存活更久.rs editable ignore mdbook-runnable}}

## 理解非词法生命周期



引用的借用范围通常在最后一次使用后结束，而不一定持续到代码块末尾。这称为非词法生命周期。

{{#playground demo/src/bin/nll_最后一次使用后结束借用.rs editable}}

## 分别借用互不重叠的字段



编译器能够识别结构体中互不重叠的字段，并允许同时创建多个可变引用。

{{#playground demo/src/bin/disjoint_fields_分别借用结构体字段.rs editable}}

## 缩小借用范围解决冲突



遇到借用冲突时，先检查引用是否存活过久。通过代码块、调整语句顺序或减少保存引用，通常可以表达真实的数据访问顺序。

{{#playground demo/src/bin/borrow_scope_缩小借用范围.rs editable}}

## 处理 Borrow Checker 报错

1. 确认值的所有者是谁。
2. 确认每个引用需要读取还是修改。
3. 找到引用最后一次实际使用的位置。
4. 缩小借用范围或拆分数据结构。
5. 只有确实需要独立所有权时才使用 `clone`。
