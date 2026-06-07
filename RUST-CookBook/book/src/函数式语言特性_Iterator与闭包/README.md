# 函数式语言特性：Iterator 与闭包

> 《Rust 程序设计语言》原文：[函数式语言特性：Iterator 与闭包](https://doc.rust-lang.org/book/ch13-00-functional-features.html)。

## 本章解决什么问题

闭包把行为保存为值，Iterator 把“如何逐项处理数据”组合成惰性流水线。二者结合后，可以用类型安全、可复用的方式表达筛选、转换、聚合与延迟计算。

## 本章知识地图

| 学习目标 | 对应章节 |
| --- | --- |
| 定义闭包并理解环境捕获 | 闭包 |
| 创建、转换并消费 Iterator | 使用 Iterator 处理一系列项 |
| 使用闭包与 Iterator 重构真实程序 | 改进 I/O 项目 |
| 理解 Iterator 的零成本抽象 | 循环与 Iterator 的性能对比 |
| 用 Iterator 组合代替手动状态管理 | 优先考虑 Iterator 转换而非显式循环 |
| 将 `Option` 视为零个或一个元素 | 对 `Option` 进行迭代 |
| 为 `move` 闭包准备独立变量 | 向闭包传递变量 |
| 使用函数指针与返回闭包 | 高级函数与闭包 |

