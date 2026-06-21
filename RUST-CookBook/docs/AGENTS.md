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
- `book/src/<顶级章节>/<知识点>/snippets/`：不参与 Cargo workspace 的失败示例或辅助代码片段。
- `target/`：Cargo 与 mdBook 构建产物。

## 目录结构

章节名称和层级直接采用中文目录，不添加 `ch03-01` 等编号前缀：

```text
book/src/
└── RUST基础/
    ├── README.md
    └── 变量与可变性/
        ├── README.md
        ├── demo/
            ├── Cargo.toml
            └── src/bin/
        └── snippets/
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
- 不自行创造或引入 Rust 中不存在的英文术语；英文只用于 Rust 原生关键字、语法、类型、Trait、函数、方法和工具名称。
- Rust 原生词汇的拼写与大小写必须和实际代码保持一致，例如 Trait 使用 `From`、方法使用 `from`。
- 标题中除 Rust 原生词汇外，其余概念使用中文表达，例如使用 `as 转换`，不使用自行引入的 `as Cast`。
- 标题中的英文可以是 Rust 原生词汇，也可以是 RAII 等行业公认的缩写或术语。
- 仅用于描述语言现象的英文概念不应替代中文宾语；例如标题使用“变量遮蔽”，不使用 `shadowing`。
- 推荐：`优先选择Option和Result转换`。
- 不推荐：`优先选择选项和结果转换`。
- Cargo package 名、Cargo binary 名和 Rust 代码标识符使用英文，并遵循 Rust 命名规范。
- `.rs` 示例文件使用中英混合名称，确保能够直接识别示例含义。
- 推荐：`shadowing_变量遮蔽.rs`。
- 不推荐：`shadowing.rs`。

## Markdown 与 Demo

- Markdown 应保持标准、清晰的块级结构，避免不同内容在 mdBook 生成的 HTML 中意外合并。
- 标题、正文段落、引用块、列表、表格和代码块之间应保留一个空行。
- 知识点小标题应直接表达该节的学习目标，并包含最重要的 Rust feature、关键字、类型、Trait、函数或工具英文名称。
- 小标题优先使用“动作 + 核心 Rust 语法”的形式，做到短小精悍，让读者仅看标题即可明确示例目标。
- 推荐：`使用 let 声明不可变绑定`、`使用 mut 重新赋值`、`使用变量遮蔽创建新绑定`。
- 不推荐：`变量与绑定`、`可变性`、`变量的相关知识与使用方法`。
- 连续的来源引用必须彼此保留一个空行，确保渲染为独立引用块。
- `《Rust 程序设计语言》` 与 `《Rust Reference》` 的每一条来源链接都单独使用一个引用块，不得连续紧贴书写。
- 推荐：

  ```markdown
  > 《Rust 程序设计语言》：[变量与可变性](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)。

  > 《Rust Reference》：[变量](https://doc.rust-lang.org/reference/variables.html)。

  > 《Rust Reference》：[下划线表达式](https://doc.rust-lang.org/reference/expressions/underscore-expr.html)。
  ```

- 不推荐：

  ```markdown
  > 《Rust Reference》：[变量](https://doc.rust-lang.org/reference/variables.html)。
  > 《Rust Reference》：[下划线表达式](https://doc.rust-lang.org/reference/expressions/underscore-expr.html)。
  ```

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
- Markdown 中不允许直接编写包含 `fn main` 的代码块；完整示例必须拆分到外部 `.rs` 文件，并通过文件方式引用。
- 不在 Markdown 中单独增加“配套 Demo”清单章节。
- 能使用断言表达示例预期时，优先使用断言，不使用无意义的打印输出。
- 所有 Demo 应能够通过 `cargo check --workspace` 检查。
- Markdown 中的 Rust 代码应尽量能够通过 `mdbook test` 检查。
- 代码中的中文注释，不应该有句号
## 知识点内容模板

- 知识点正文应提炼官方资料的核心规则，使用稳定的信息层次组织内容，避免连续堆叠大段解释。
- 模板中的栏目按实际内容选用，不要求每个知识点机械包含全部栏目。
- 栏目名称必须表达实际含义，不使用 `扩展1`、`扩展2` 等无法独立说明内容的名称。

### 语法特性模板

- 适用于 `let`、`mut`、`const`、Shadowing 等语言特性。
- 按需使用：`核心规则`、`基本示例`、`适用范围`、`语法限制`、`实践建议`。

### 特性对比模板

- 适用于 `mut` 与 Shadowing、`String` 与 `&str` 等特性选择。
- 按需使用：`核心区别`、`选择原则`、`对比示例`。
- 核心区别适合使用表格表达，选择原则适合使用简短列表表达。

### 错误与限制模板

- 适用于所有权错误、借用冲突、类型不匹配等失败场景。
- 按需使用：`错误原因`、`错误示例`、`正确做法`。
- 错误示例紧跟原因，正确做法应引用可以运行或对照学习的示例。

### 最佳实践模板

- 适用于总结一组语法或 API 的选择建议。
- 按需使用：`默认选择`、`选择建议`、`注意事项`。
- 建议必须具体、可执行，避免重复正文或只表达宽泛原则。

## 错误与失败示例

- Panic、编译失败等错误情况可以在 Markdown 中展示给读者。
- 编译失败、Panic 或不适合作为 Cargo target 的示例放入知识点目录下的 `snippets/`。
- `snippets/` 不加入 Cargo workspace，不得影响 `cargo check --workspace`。
- Markdown 通过文件方式引用 `snippets/` 中的 `.rs` 文件，让读者在 mdBook 页面运行并查看错误。
- 编译失败示例引用时添加 `ignore mdbook-runnable` 属性：`ignore` 避免阻塞 `mdbook test`，`mdbook-runnable` 保留页面运行按钮。
- 不为编译失败示例创建或维护 `output.txt`，也不在 Markdown 中渲染编译器输出文件。
- 失败示例应紧跟对应知识说明，并采用能够说明失败原因的中英混合文件名。

示例：

```markdown
{{#playground snippets/immutable_不可变绑定不能重新赋值.rs editable ignore mdbook-runnable}}
```

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

## markdown语法规约
### 用>表示"下定义"
```markdown
## Rust语言
### Rust语言的特点是什么
> 在没有垃圾回收 GC 的情况下，尽量保证内存安全和线程安全
```
### 普通正文之前不加任何符号
```markdown
## 二级标题
正文...
```
## rust-demo代码规约
1. 注释不需要中文句号