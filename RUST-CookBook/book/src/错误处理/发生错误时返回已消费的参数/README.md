# 发生错误时返回已消费的参数

> 本文整理自 Rust Design Patterns 的 Idiom：[Return consumed arg on error](https://rust-unofficial.github.io/patterns/idioms/return-consumed-arg-on-error.html)。

归入“错误处理”，因为失败时返还被消费参数可以让调用者恢复所有权并决定后续处理。

## 待补充

- 自定义Error携带输入值
- API权衡
- 配套Demo与实践检查表
