# 使用 String 存储 UTF-8 编码文本

> 《Rust 程序设计语言》：[使用 String 存储 UTF-8 编码文本](https://doc.rust-lang.org/book/ch08-02-strings.html)。

## 使用 `String` 存储可增长文本

{{#playground demo/src/bin/string_创建并追加文本.rs editable}}

## 使用 `chars` 或 `bytes` 遍历文本

`String` 使用 UTF-8 编码，不能使用整数直接索引字符。

{{#playground demo/src/bin/string_chars_按字符与字节遍历.rs editable}}

{{#playground snippets/string_不能使用整数索引.rs editable ignore mdbook-runnable}}

## 使用 `&str` 借用字符串内容

{{#playground demo/src/bin/string_slice_借用字符串内容.rs editable}}

