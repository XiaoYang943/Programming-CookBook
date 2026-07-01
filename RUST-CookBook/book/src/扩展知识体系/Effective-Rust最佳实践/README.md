# Effective Rust最佳实践

[Effective Rust](https://effective-rust.com/title-page.html)总结了35条编写高质量Rust代码的工程实践。本项目不按原书章节重复存放内容，而是将每一条建议归入最相关的知识章节。

每个页面都会注明对应的Effective Rust Item编号和可跳转的原文链接。

## 35条建议索引

| Item | 建议 | 本项目归属                                                                                                      |
| ---: | --- |------------------------------------------------------------------------------------------------------------|
| 1 | [Use the type system to express your data structures](https://effective-rust.com/use-types.html) | [使用类型系统表达数据结构](../../RUST基础/使用类型系统表达数据结构/README.md)                                                        |
| 2 | [Use the type system to express common behavior](https://effective-rust.com/use-types-2.html) | [使用类型系统表达共同行为](../../Trait与共同行为/使用类型系统表达共同行为/README.md)                                                 |
| 3 | [Prefer Option and Result transforms over explicit match expressions](https://effective-rust.com/transform.html) | [优先选择Option和Result转换](../../错误处理/优先选择Option和Result转换/README.md)                                            |
| 4 | [Prefer idiomatic Error types](https://effective-rust.com/errors.html) | [优先使用符合Rust惯例的Error类型](../../错误处理/优先使用符合Rust惯例的Error类型/README.md)                                          |
| 5 | [Understand type conversions](https://effective-rust.com/casts.html) | [理解类型转换](../../RUST基础/理解类型转换/README.md)                                                                    |
| 6 | [Embrace the newtype pattern](https://effective-rust.com/newtype.html) | [使用Newtype模式](../../高级特性/使用Newtype模式/README.md)                                                            |
| 7 | [Use builders for complex types](https://effective-rust.com/builders.html) | [为复杂类型使用 Builder 模式](../../Rust设计模式/创建型模式_Creational/Builder构建器模式/复杂类型Builder实践/README.md)                                             |
| 8 | [Familiarize yourself with reference and pointer types](https://effective-rust.com/references.html) | [值的使用方式](../../所有权/值的使用方式/README.md)                                                                    |
| 9 | [Consider iterator transforms instead of explicit loops](https://effective-rust.com/iterators.html) | [优先考虑Iterator转换而非显式循环](../../迭代器与闭包/优先考虑Iterator转换而非显式循环/README.md)                           |
| 10 | [Familiarize yourself with standard traits](https://effective-rust.com/std-traits.html) | [熟悉标准Trait](../../Trait与共同行为/熟悉标准Trait/README.md)                                                       |
| 11 | [Implement the Drop trait for RAII patterns](https://effective-rust.com/raii.html) | [通过实现Drop-Trait清理资源](../../智能指针/通过实现Drop-Trait清理资源/README.md)                                          |
| 12 | [Understand the trade-offs between generics and trait objects](https://effective-rust.com/generics.html) | [理解泛型与Trait Object的权衡](../../Trait与共同行为/理解泛型与Trait-Object的权衡/README.md)                                 |
| 13 | [Use default implementations to minimize required trait methods](https://effective-rust.com/default-impl.html) | [使用默认实现减少Trait必需方法](../../Trait与共同行为/使用默认实现减少Trait必需方法/README.md)                                       |
| 14 | [Understand lifetimes](https://effective-rust.com/lifetimes.html) | [理解生命周期](../../生命周期/理解生命周期/README.md)                                                             |
| 15 | [Understand the borrow checker](https://effective-rust.com/borrows.html) | [理解Borrow Checker](../../所有权/理解Borrow-Checker/README.md)                                                   |
| 16 | [Avoid writing unsafe code](https://effective-rust.com/unsafe.html) | [避免编写Unsafe代码](../../高级特性/避免编写Unsafe代码/README.md)                                                          |
| 17 | [Be wary of shared-state parallelism](https://effective-rust.com/deadlock.html) | [警惕共享状态并行](../../并发编程/警惕共享状态并行/README.md)                                                                  |
| 18 | [Don't panic](https://effective-rust.com/panic.html) | [不要轻易panic](../../错误处理/不要轻易panic/README.md)                                                                |
| 19 | [Avoid reflection](https://effective-rust.com/reflection.html) | [避免使用反射](../../Trait与共同行为/避免使用反射/README.md)                                                             |
| 20 | [Avoid the temptation to over-optimize](https://effective-rust.com/optimize.html) | [避免过早和过度优化](../避免过早和过度优化/README.md)                                                                        |
| 21 | [Understand what semantic versioning promises](https://effective-rust.com/semver.html) | [理解Semantic Versioning的承诺](../../Package_Crate_Module与Cargo/Crate设计与发布/理解Semantic-Versioning的承诺/README.md) |
| 22 | [Minimize visibility](https://effective-rust.com/visibility.html) | [最小化可见性](../../Package_Crate_Module与Cargo/Crate设计与发布/最小化可见性/README.md)                                     |
| 23 | [Avoid wildcard imports](https://effective-rust.com/wildcard.html) | [避免使用通配符导入](../../Package_Crate_Module与Cargo/Crate设计与发布/避免使用通配符导入/README.md)                               |
| 24 | [Re-export dependencies whose types appear in your API](https://effective-rust.com/re-export.html) | [重新导出公共API中出现的依赖类型](../../Package_Crate_Module与Cargo/Crate设计与发布/重新导出公共API中出现的依赖类型/README.md)               |
| 25 | [Manage your dependency graph](https://effective-rust.com/dep-graph.html) | [管理依赖关系图](../../Package_Crate_Module与Cargo/Cargo项目管理/管理依赖关系图/README.md)                                    |
| 26 | [Be wary of feature creep](https://effective-rust.com/features.html) | [警惕Feature功能膨胀](../../Package_Crate_Module与Cargo/Cargo项目管理/警惕Feature功能膨胀/README.md)                        |
| 27 | [Document public interfaces](https://effective-rust.com/documentation.html) | [为公共接口编写文档](../为公共接口编写文档/README.md)                                                                        |
| 28 | [Use macros judiciously](https://effective-rust.com/macros.html) | [谨慎使用宏](../../宏_Macros/谨慎使用宏/README.md)                                                                    |
| 29 | [Listen to Clippy](https://effective-rust.com/clippy.html) | [听取Clippy的建议](../听取Clippy的建议/README.md)                                                                    |
| 30 | [Write more than unit tests](https://effective-rust.com/testing.html) | [不要只编写单元测试](../../编写自动化测试/不要只编写单元测试/README.md)                                                             |
| 31 | [Take advantage of the tooling ecosystem](https://effective-rust.com/use-tools.html) | [充分利用Rust工具生态](../充分利用Rust工具生态/README.md)                                                                  |
| 32 | [Set up a continuous integration system](https://effective-rust.com/ci.html) | [建立持续集成CI系统](../../编写自动化测试/建立持续集成CI系统/README.md)                                                           |
| 33 | [Consider making library code compatible with no_std](https://effective-rust.com/no-std.html) | [考虑让库兼容no_std](../../高级特性/考虑让库兼容no_std/README.md)                                                          |
| 34 | [Control what crosses FFI boundaries](https://effective-rust.com/ffi.html) | [控制跨越FFI边界的内容](../../FFI与跨语言交互/控制跨越FFI边界的内容/README.md)                                                     |
| 35 | [Prefer bindgen over manual FFI mappings](https://effective-rust.com/bindgen.html) | [优先使用bindgen生成FFI映射](../../FFI与跨语言交互/优先使用bindgen生成FFI映射/README.md)                                         |
