# Builder构建器模式

> 本文整理自 Rust Design Patterns：[Builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)。

Builder 模式适合构造参数较多、可选配置较多，或构造过程需要校验的复杂类型。它通常通过链式配置逐步收集参数，最后在 `build` 阶段生成目标值。

- [复杂类型 Builder 实践](复杂类型Builder实践/README.md)

## 后续可扩展方向

- 必填字段与类型状态 Builder
- Builder 与默认值、校验错误的权衡
- Builder API 的可读性与兼容性设计