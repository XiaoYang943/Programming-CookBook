# 注释

> 《Rust 程序设计语言》原文：[注释](https://doc.rust-lang.org/book/ch03-04-comments.html)。

> 《Rust Reference》：[注释](https://doc.rust-lang.org/reference/comments.html)。

## 使用 `//` 编写行注释

**核心规则**

`//` 从当前位置注释到行尾。行注释适合解释设计原因、约束和不明显的行为。

{{#playground demo/src/bin/line_comment_行注释.rs editable}}

## 使用 `/* */` 编写嵌套块注释

**核心规则**

块注释使用 `/* */`，并且支持嵌套。它适合临时包围较大区域，但普通说明通常优先使用行注释。

{{#playground demo/src/bin/block_comment_嵌套块注释.rs editable}}

## 使用文档注释编写公共接口文档

**核心规则**

`///` 为后面的项编写文档，`//!` 为包含它的项编写文档。Doc Comment 支持 Markdown，并由 rustdoc 处理。

{{#playground demo/src/bin/doc_comment_API文档.rs editable}}

**实践建议**

- 注释重点解释“为什么”，不要逐行复述代码。
- 代码行为变化时同步更新注释。
- 公共 API 的使用方式和约束使用 Doc Comment 表达。
