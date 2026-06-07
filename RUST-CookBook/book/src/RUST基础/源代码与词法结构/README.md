# 源代码与词法结构

> 《Rust Reference》：[词法结构](https://doc.rust-lang.org/reference/lexical-structure.html)。

> 《Rust Reference》：[输入格式](https://doc.rust-lang.org/reference/input-format.html)。

> 《Rust Reference》：[Shebang](https://doc.rust-lang.org/reference/shebang.html)。

> 《Rust Reference》：[标识符](https://doc.rust-lang.org/reference/identifiers.html)。

> 《Rust Reference》：[空白字符](https://doc.rust-lang.org/reference/whitespace.html)。

> 《Rust Reference》：[Token](https://doc.rust-lang.org/reference/tokens.html)。

## 使用标识符命名代码元素



Identifier（标识符）用于命名变量、函数、类型和 Module 等代码元素。Rust 标识符区分大小写，并支持 Unicode。

{{#playground demo/src/bin/identifier_标识符与命名.rs editable}}

**实践建议**

项目代码优先遵循 Rust 命名惯例：变量和函数使用 `snake_case`，类型使用 `UpperCamelCase`，常量使用 `SCREAMING_SNAKE_CASE`。

## 使用原始标识符引用关键字名称



Raw Identifier 使用 `r#name` 语法，让关键字能够作为标识符使用，主要用于 Edition 兼容和外部接口。

{{#playground demo/src/bin/raw_identifier_使用关键字名称.rs editable}}

**实践建议**

普通业务命名不要主动使用关键字；仅在兼容已有 API 时使用 Raw Identifier。

## 理解词法单元与空白字符



编译器将源码识别为 Token。空格、换行和注释通常用于分隔 Token，不会改变表达式的计算结果。

{{#playground demo/src/bin/tokens_空白分隔Token.rs editable}}

**语法限制**

不能随意在一个 Token 内部插入空白字符。

{{#playground snippets/tokens_不能拆分单个Token.rs editable ignore mdbook-runnable}}

## 使用字面量表达固定值



Literal（字面量）直接表示固定值，例如整数、浮点数、字符、字符串、布尔值和字节。

{{#playground demo/src/bin/literals_常用字面量.rs editable}}
