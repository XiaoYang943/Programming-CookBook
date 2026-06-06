# RUST-CookBook 项目约定

## 项目目标

本项目用于系统学习 Rust，并逐步积累个人 Rust 知识体系。

- 使用 mdBook 组织教程、笔记、知识地图、最佳实践与资料索引。
- 使用 Cargo workspace 管理与知识点就近存放、可运行、可调试、可测试的 Demo。
- 暂不使用 rustdoc 组织知识库，但需要在 Rust 生态章节中介绍 rustdoc。
- 知识框架先保持完整，章节内容和 Demo 后续逐步补充。
- 以 `D:\code\source\book\src\SUMMARY-CN.md` 的中文章节顺序和父子关系作为知识主干。

## 目录职责

- `book/src/`：mdBook Markdown 知识库。
- `book/src/<顶级章节>/README.md`：顶级章节入口。
- `book/src/<顶级章节>/<知识点>/README.md`：知识点正文。
- `book/src/<顶级章节>/<知识点>/demo/`：与知识点就近存放的可选 Cargo Package。
- `target/`：Cargo 与 mdBook 构建产物。

## 目录结构

章节名称和层级直接采用中文目录，不添加 `ch03-01` 等编号前缀：

```text
book/src/
└── 常见编程概念/
    ├── README.md
    └── 变量与可变性/
        ├── README.md
        └── demo/
            ├── Cargo.toml
            └── src/bin/
```

- 不创建全局 `demos/` 目录。
- 不创建额外的 `docs/` 目录。
- 每个知识点的 Markdown 与可调试 Demo 放在同一个知识点目录中。
- 没有代码示例的知识点可以只有 `README.md`，无需创建 `demo/`。
- 没有子章节的顶级章节如果需要Demo，统一放入 `<顶级章节>/示例/demo/`，以保持Workspace匹配深度一致。
- 根 Cargo workspace 使用 `members = ["book/src/*/*/demo"]` 自动发现 Demo。

## 命名规范

- 目录名称、Markdown 文件名称应优先使用中文或中英混合，确保一眼可以理解内容。
- Rust 专有名词、类型、Trait、函数和工具名称保留英文，不做生硬翻译。
- 推荐：`优先选择Option和Result转换`。
- 不推荐：`优先选择选项和结果转换`。
- Cargo package 名、Cargo binary 名和 Rust 代码标识符使用英文，并遵循 Rust 命名规范。
- `.rs` 示例文件使用中英混合名称，确保能够直接识别示例含义。
- 推荐：`shadowing_变量遮蔽.rs`。
- 不推荐：`shadowing.rs`。

## Markdown 与 Demo

- 每个知识主题应逐步具备原理说明、最小 Demo、常见错误、最佳实践和参考资料。
- Effective Rust等最佳实践应归入最相关的现有知识章节，不要全部集中堆放在扩展目录。
- 每个归档的Effective Rust页面必须注明Item编号，并附上可跳转的原文链接。
- `扩展知识体系/Effective-Rust最佳实践/README.md`只作为35条建议的统一索引和反向导航。
- Rust Design Patterns中的Idioms与Anti-patterns应归入最相关的现有知识章节，并在页面中注明来源类型和原文链接。
- Rust Design Patterns是独立一级模块，Design Patterns集中维护在`book/src/Rust设计模式/`专题中。
- `book/src/Rust设计模式/反模式_Anti-Patterns/README.md`只作为反模式统一索引，正文按知识点分散归档。
- 异步编程是独立一级模块，统一维护在`book/src/异步编程_Async/`，整合Rust Book异步基础与Async Book深入专题。
- Async Book仍在持续重写，异步模块按稳定知识主题组织，不机械照搬其临时目录结构。
- 简单示例也放入知识点的 `demo/` Cargo Package，确保可以在 RustRover 中运行和断点调试。
- 单文件、仅依赖标准库的示例，同时应允许在 mdBook 页面通过 Rust Playground 运行。
- 多文件、第三方依赖、网络、异步运行时等示例，通过 Cargo 命令运行。
- mdBook 中的示例应尽量引用真实 Demo 文件，避免文档代码与 Demo 长期不一致。
- Markdown 正文负责总结知识点，代码示例直接出现在对应知识点附近，并引用真实 `.rs` 文件。
- 不在 Markdown 中单独增加“配套 Demo”清单章节。
- 能使用断言表达示例预期时，优先使用断言，不使用无意义的打印输出。
- 所有 Demo 应能够通过 `cargo check --workspace` 检查。
- Markdown 中的 Rust 代码应尽量能够通过 `mdbook test` 检查。

## 错误与失败示例

- Panic、编译失败等错误情况可以在 Markdown 中展示给读者。
- 编译失败示例放入知识点目录下的 `compile_fail/`，不得加入正常 Cargo workspace。
- 编译失败代码使用 `rust,compile_fail` 代码块。
- 编译器或程序输出保存为示例目录下的 `output.txt`。
- `output.txt` 第一行记录执行命令，例如 `$ cargo check`。
- `output.txt` 应清理本地绝对路径、耗时和其他不稳定信息，避免环境差异干扰阅读。
- 使用 `console` 代码块和 `{{#include ...}}` 引用 `output.txt`。

示例：

````markdown
```rust,compile_fail
{{#include compile_fail/immutable_不可变变量不能重新赋值/src/main.rs}}
```

```console
{{#include compile_fail/immutable_不可变变量不能重新赋值/output.txt}}
```
````

## 修改与删除约束

- 每次对话涉及修改现有代码或配置前，必须先向用户确认。
- 新增文件和目录不必询问用户。
- 禁止批量删除文件或目录。
- 禁止使用 `del /s`、`rd /s`、`rmdir /s`、`Remove-Item -Recurse`、`rm -rf`。
- 删除文件时，只能一次删除一个明确路径的文件。
- 如果需要批量删除，应停止操作并让用户手动删除。

## 主要参考资料

- Rust 圣经：https://beatai.org/rust-course/about-book
- The Rust Programming Language：https://github.com/rust-lang/book/blob/main/src/SUMMARY.md
- Rust Performance Book：https://nnethercote.github.io/perf-book/title-page.html
- Async Book：https://rust-lang.github.io/async-book/intro.html
- Rust API Guidelines：https://rust-lang.github.io/api-guidelines/naming.html
- Rustonomicon：https://doc.rust-lang.org/nomicon/intro.html
- Effective Rust：https://effective-rust.com/types.html
- Rust Design Patterns：https://rust-unofficial.github.io/patterns/intro.html
- Rust std library API doc: https://doc.rust-lang.org/stable/std/index.html
