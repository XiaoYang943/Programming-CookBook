# 循环与 Iterator 的性能对比

> 《Rust 程序设计语言》：[循环与 Iterator 的性能对比](https://doc.rust-lang.org/book/ch13-04-performance.html)。

## 对比循环与 Iterator 的结果

显式循环与 Iterator 链通常能够优化为相近的机器代码。选择时应先关注表达是否清晰，再用基准测试确认热点。

{{#playground demo/src/bin/loop_iterator_计算结果保持一致.rs editable}}

## 理解零成本抽象

Iterator 的泛型与闭包通常通过单态化生成具体代码；适配器组合可以被内联，惰性求值也避免了不必要的中间集合。

{{#playground demo/src/bin/lazy_避免创建中间集合.rs editable}}

## 在性能优化前测量

- 使用发布构建评估性能：`cargo bench` 或可靠的基准测试工具。
- 比较完整工作负载，不用单次运行时间下结论。
- 当 Iterator 链影响可读性或测量结果确实更差时，再改用显式循环。

