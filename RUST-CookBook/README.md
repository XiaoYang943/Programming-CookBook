# RUST-CookBook

这是一个用于系统学习 Rust、积累学习笔记和可运行 Demo 的个人知识库。

- 使用 mdBook 组织 Markdown 知识体系。
- 使用 Cargo workspace 管理可运行、可测试、可断点调试的 Demo。
- Markdown 与对应 Demo 就近存放，并尽量引用同一份真实代码。

## 安装 mdBook

通过 Cargo 安装：

```powershell
cargo install mdbook
```

检查是否安装成功：

```powershell
mdbook --version
```

mdBook 文档：

<https://rust-lang.github.io/mdBook/>

## 本地预览知识库

在项目根目录执行：

```powershell
mdbook serve --open
```

该命令会：

- 构建 mdBook。
- 启动本地 Web 服务。
- 在浏览器中打开知识库。
- 监视 Markdown 文件变化并自动重新构建。

默认访问地址：

```text
http://localhost:3000
```

如果不希望自动打开浏览器：

```powershell
mdbook serve
```

## 构建 mdBook

执行：

```powershell
mdbook build
```

生成的 HTML 位于：

```text
target/mdbook/
```

入口页面：

```text
target/mdbook/index.html
```

## 测试 Markdown 中的 Rust 代码

执行：

```powershell
mdbook test
```

该命令会编译和测试 Markdown 中能够执行的 Rust 代码块，以及通过 `{{#playground}}` 引用的 Rust 文件。

建议修改知识笔记或 Demo 后运行：

```powershell
mdbook test
cargo check --workspace
```

## 项目目录结构

知识体系以《Rust 程序设计语言》的中文章节结构为主干：

```text
book/
└── src/
    ├── SUMMARY.md
    ├── 常见编程概念/
    │   ├── README.md
    │   └── 变量与可变性/
    │       ├── README.md
    │       └── demo/
    │           ├── Cargo.toml
    │           └── src/bin/
    │               └── mutable.rs
    └── 扩展知识体系/
```

其中：

- `book/src/SUMMARY.md`：控制 mdBook 的章节顺序和导航结构。
- 顶级章节的 `README.md`：章节入口。
- 知识点目录中的 `README.md`：知识点正文。
- 知识点目录中的 `demo/`：可选的 Cargo Package，用于运行和调试代码。

## 新增知识章节

例如新增“常见编程概念/表达式”：

```text
book/src/常见编程概念/表达式/
└── README.md
```

然后在 `book/src/SUMMARY.md` 中添加：

```markdown
- [表达式](常见编程概念/表达式/README.md)
```

只有被 `SUMMARY.md` 引用的 Markdown 页面才会显示在 mdBook 导航中。

## 为知识点创建可调试 Demo

需要代码示例时，在知识点目录中创建固定名称的 `demo` Cargo Package：

```powershell
cargo new "book/src/常见编程概念/表达式/demo" --name expressions_demo
```

建议将多个独立示例放入 `src/bin/`：

```text
表达式/
├── README.md
└── demo/
    ├── Cargo.toml
    └── src/bin/
        ├── statements.rs
        └── expressions.rs
```

每个 `.rs` 文件需要包含自己的 `main()` 函数。

根目录 `Cargo.toml` 使用以下配置自动发现 Demo：

```toml
[workspace]
members = ["book/src/*/*/demo"]
resolver = "2"
```

因此，不需要手动将每个 Demo 添加到 workspace。

运行指定 Demo：

```powershell
cargo run -p expressions_demo --bin expressions
```

检查全部 Demo：

```powershell
cargo check --workspace
```

在 RustRover 中可以打开 `src/bin/*.rs`，设置断点后点击 `main()` 函数旁的调试按钮。

## 在 mdBook 页面中展示并运行真实 Demo

知识点的 `README.md` 可以直接引用同目录 Demo：

```markdown
{{#playground demo/src/bin/expressions.rs editable}}
```

这样同一份代码可以：

- 在 mdBook 页面展示和编辑。
- 通过 Rust Playground 在线运行。
- 通过 `mdbook test` 检查。
- 通过 Cargo 编译和运行。
- 在 RustRover 中断点调试。

页面运行依赖在线 Rust Playground，适合单文件和标准库示例。包含本地文件、复杂依赖、网络服务或异步运行时的示例，应通过 Cargo 和 RustRover 运行。

## 常用命令

```powershell
# 启动本地知识库
mdbook serve --open

# 构建静态HTML
mdbook build

# 测试Markdown中的Rust代码
mdbook test

# 检查所有Cargo Demo
cargo check --workspace

# 运行指定Demo
cargo run -p variables_and_mutability --bin mutable
```

## 主要配置文件

- `book.toml`：mdBook 配置，包括源码路径、输出目录和 Playground 设置。
- `book/src/SUMMARY.md`：知识库目录和导航结构。
- `Cargo.toml`：根 Cargo workspace 配置。
- `AGENTS.md`：项目目标、目录结构、命名规则和操作约束。
