# 改进 I/O 项目

> 《Rust 程序设计语言》原文：[改进 I/O 项目](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html)。

## 使用 Iterator 读取参数

让配置构造函数接收 `Iterator<Item = String>`，可以消除索引访问，并让测试直接传入内存中的参数。

{{#playground demo/src/bin/args_使用Iterator解析参数.rs editable}}

## 使用 Iterator 实现文本搜索

`lines`、`filter` 与 `collect` 可以直接表达“逐行筛选匹配项”，避免手动维护结果集合。

{{#playground demo/src/bin/search_使用Iterator筛选文本行.rs editable}}

## 使用闭包切换搜索策略

把差异封装为闭包，公共的 Iterator 流程只需保留一份。

{{#playground demo/src/bin/closure_切换大小写搜索策略.rs editable}}

## 实践建议

- 配置解析函数接收 Iterator，使输入来源与业务逻辑解耦。
- 纯转换逻辑返回值，不在内部打印，便于断言与复用。
- Iterator 链较长时，为关键阶段命名或拆成小函数。

