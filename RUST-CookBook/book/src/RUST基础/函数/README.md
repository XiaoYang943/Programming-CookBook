# 函数

> 《Rust 程序设计语言》：[函数](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)。

> 《Rust Reference》：[函数声明](https://doc.rust-lang.org/reference/items/functions.html)。

> 《Rust Reference》：[调用表达式](https://doc.rust-lang.org/reference/expressions/call-expr.html)。

> 《Rust Reference》：[Return 表达式](https://doc.rust-lang.org/reference/expressions/return-expr.html)。

## 使用 `fn` 声明函数



使用 `fn` 声明函数。函数和参数使用 `snake_case` 命名，每个参数都必须标注类型。

{{#playground demo/src/bin/function_parameters_声明函数与参数.rs editable}}

## 使用尾表达式返回值



函数体是块表达式。最后一个没有分号的表达式会成为函数返回值。

{{#playground demo/src/bin/tail_expression_尾表达式返回值.rs editable}}

**语法限制**

在尾表达式后添加分号会把表达式变成语句，其结果是单元类型 `()`。

{{#playground snippets/tail_expression_分号导致返回类型错误.rs editable ignore mdbook-runnable}}

## 使用 `return` 提前返回



通常使用尾表达式返回最终结果；需要提前结束函数时使用 `return`。

{{#playground demo/src/bin/return_提前返回.rs editable}}

## 区分语句与表达式

| 结构 | 是否产生值 | 示例 |
| --- | --- | --- |
| Statement | 通常不产生可使用的值 | `let value = 5;` |
| Expression | 计算并产生值 | `5 + 1`、代码块、`if` |

**实践建议**

- 保持函数职责单一，使用名称表达意图。
- 参数和返回类型用于明确函数契约。
- 优先使用尾表达式返回最终结果，使用 `return` 表达提前退出。
