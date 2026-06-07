# match 控制流结构

> 《Rust 程序设计语言》：[match 控制流结构](https://doc.rust-lang.org/book/ch06-02-match.html)。

## 使用 `match` 穷尽处理所有变体

`match` 必须覆盖所有可能情况，编译器会阻止遗漏分支。

{{#playground demo/src/bin/match_穷尽处理变体.rs editable}}

{{#playground snippets/match_遗漏分支无法编译.rs editable ignore mdbook-runnable}}

## 使用模式绑定提取数据

{{#playground demo/src/bin/match_binding_提取变体数据.rs editable}}

## 使用 `_` 处理其余情况

只关心部分情况时，可以使用 `_` 兜底；需要使用兜底值时绑定一个变量。

{{#playground demo/src/bin/match_wildcard_处理其余情况.rs editable}}

