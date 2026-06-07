# 控制流

> 《Rust 程序设计语言》原文：[控制流](https://doc.rust-lang.org/book/ch03-05-control-flow.html)。

> 《Rust Reference》：[语句与表达式](https://doc.rust-lang.org/reference/statements-and-expressions.html)。

> 《Rust Reference》：[语句](https://doc.rust-lang.org/reference/statements.html)。

> 《Rust Reference》：[表达式](https://doc.rust-lang.org/reference/expressions.html)。

> 《Rust Reference》：[块表达式](https://doc.rust-lang.org/reference/expressions/block-expr.html)。

> 《Rust Reference》：[分组表达式](https://doc.rust-lang.org/reference/expressions/grouped-expr.html)。

> 《Rust Reference》：[If 表达式](https://doc.rust-lang.org/reference/expressions/if-expr.html)。

> 《Rust Reference》：[循环表达式](https://doc.rust-lang.org/reference/expressions/loop-expr.html)。

> 《Rust Reference》：[范围表达式](https://doc.rust-lang.org/reference/expressions/range-expr.html)。

## 使用 `if` 表达式选择值



`if` 的条件必须是 `bool`。作为表达式使用时，每个分支必须产生兼容类型的值。

{{#playground demo/src/bin/if_expression_使用条件选择值.rs editable}}

{{#playground snippets/if_expression_分支类型必须一致.rs editable ignore mdbook-runnable}}

## 使用 `loop` 重复并通过 `break` 返回值



`loop` 会持续执行，直到通过 `break`、`return` 或 panic 退出。`break value` 可以让循环表达式产生值。

{{#playground demo/src/bin/loop_break_循环返回值.rs editable}}

## 使用 `while` 按条件循环



当重复次数取决于运行时条件时，使用 `while`。

{{#playground demo/src/bin/while_按条件循环.rs editable}}

## 使用 `for` 遍历集合与范围



遍历集合或范围时优先使用 `for`，它能避免手动索引带来的越界风险。

{{#playground demo/src/bin/for_range_遍历集合与范围.rs editable}}

## 使用循环标签控制嵌套循环



循环标签以 `'name` 声明，可让 `break` 和 `continue` 明确作用于指定循环。

{{#playground demo/src/bin/loop_label_控制嵌套循环.rs editable}}

## 选择控制流结构

| 需求 | 选择 |
| --- | --- |
| 根据条件选择行为或值 | `if` |
| 无限循环或需要从循环返回值 | `loop` |
| 条件为真时持续循环 | `while` |
| 遍历集合或 Range | `for` |
