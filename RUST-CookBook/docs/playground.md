# Rust Playground 使用指南

> Rust Playground 在线地址：[play.rust-lang.org](https://play.rust-lang.org/)。
>
> 官方仓库：[rust-lang/rust-playground](https://github.com/rust-lang/rust-playground)。

Rust Playground 是一个在线 Rust 编译与运行环境。它适合快速验证语法、制作最小复现示例、分享代码，以及查看编译器生成的中间结果。

本项目中的 mdBook 也可以将 Rust 代码块发送到 Rust Playground，让读者直接在页面中运行代码。

## Playground 在线编辑器

Rust Playground 在线编辑器支持：

- 使用 Stable、Beta 或 Nightly Toolchain 编译代码。
- 使用 Debug 或 Release 模式编译代码。
- 运行 Rust 程序和测试。
- 使用 `rustfmt` 格式化代码。
- 使用 Clippy 检查代码。
- 查看编译器错误与程序标准输出。
- 查看 MIR、LLVM IR 和汇编代码。
- 创建 GitHub Gist 并分享代码链接。
- 使用一部分预装的常用 Crate。

### 推荐使用场景

- 验证一个 Rust 语法点。
- 制作问题的最小可复现示例。
- 比较 Stable、Beta 和 Nightly 的行为差异。
- 使用 Clippy 检查小段代码。
- 查看 Release 模式下生成的汇编或 LLVM IR。
- 向他人分享无需本地工程即可运行的示例。

### 不适合的场景

- 多 Crate Cargo Workspace。
- 依赖本地文件、数据库或外部服务的程序。
- 需要网络访问的程序。
- 长时间运行或消耗大量内存的程序。
- 需要自定义 `Cargo.toml`、Feature 或 Build Script 的复杂工程。
- 性能基准测试。

Playground 在隔离容器中运行，编译器容器不能访问外部网络，并且内存、编译时间和执行时间都受到限制。

## Playground 在线编辑器最佳实践

### 保持示例最小化

删除与问题无关的类型、函数和依赖，只保留能够说明问题的代码。

```rust
fn main() {
    let values = vec![1, 2, 3];
    println!("{values:?}");
}
```

### 明确 Toolchain 和构建模式

分享代码时说明使用的是 Stable、Beta 还是 Nightly，以及 Debug 还是 Release 模式。

Nightly 功能应显式添加对应的 Feature：

```rust
#![feature(example_feature)]
```

不要假设 Nightly 示例可以在 Stable 中运行。

### 使用断言表达预期

相比只打印结果，断言可以更准确地说明示例想验证什么。

```rust
fn main() {
    let result: i32 = [1, 2, 3].into_iter().sum();
    assert_eq!(result, 6);
}
```

### 分享前运行 rustfmt 和 Clippy

分享最小复现示例前，建议依次运行：

1. `rustfmt`
2. Clippy
3. Run 或 Test

格式统一并消除无关警告后，示例更容易阅读和讨论。

### 不要放入敏感信息

不要在 Playground 中粘贴 Token、密码、私有源码或其他敏感数据。创建 Gist 后，代码可能通过链接被他人访问。

## mdBook 中的 Playground

mdBook 会为可运行的 Rust 代码块添加运行按钮。点击运行按钮后，代码会发送到 Rust Playground 编译并执行，结果显示在代码块下方。

普通可运行代码块：

````markdown
```rust
fn main() {
    println!("Hello, Playground!");
}
```
````

如果代码块中没有 `fn main()`，mdBook 和 rustdoc 通常会自动将代码包装进 `fn main()`。

### Rust 代码块属性

代码块属性写在 `rust` 后面，可以使用逗号、空格或 Tab 分隔。

````markdown
```rust,editable
fn main() {
    println!("可以编辑后运行");
}
```
````

常用属性：

| 属性 | 作用 |
| --- | --- |
| `editable` | 允许读者在页面中编辑代码后运行 |
| `noplayground` | 不显示运行按钮，但仍会被 `mdbook test` 测试 |
| `mdbook-runnable` | 强制显示运行按钮，常与 `ignore` 配合 |
| `ignore` | 不执行测试，也不显示运行按钮 |
| `no_run` | 只检查编译，不运行代码 |
| `should_panic` | 示例应当成功编译，并在运行时发生 Panic |
| `edition2015` | 使用 Rust 2015 Edition |
| `edition2018` | 使用 Rust 2018 Edition |
| `edition2021` | 使用 Rust 2021 Edition |
| `edition2024` | 使用 Rust 2024 Edition |

### 隐藏辅助代码

以 `# ` 开头的行会参与编译，但默认不会展示给读者。读者可以通过展开按钮查看隐藏代码。

````markdown
```rust
# fn main() {
let answer = 40 + 2;
assert_eq!(answer, 42);
# }
```
````

需要显示以 `# ` 开头的真实代码时，使用 `##` 转义第一个 `#`。

隐藏代码适合放置：

- `fn main()` 包装。
- 必要但与当前知识点无关的 `use`。
- 测试数据和辅助函数。
- 为 `?` 运算符准备的 `Result` 返回类型。

不要隐藏理解示例所必需的核心逻辑。

### 使用 `?` 运算符

需要在示例中使用 `?` 时，可以隐藏返回 `Result` 的 `main` 函数：

````markdown
```rust
# use std::io;
# fn main() -> io::Result<()> {
let mut input = String::new();
io::stdin().read_line(&mut input)?;
# Ok(())
# }
```
````

## 引入真实 `.rs` 文件

本项目推荐将可运行代码维护在真实 `.rs` 文件中，再由 mdBook 引用。这样同一份代码既能在网页中运行，也能通过 Cargo 执行并在 RustRover 中调试。

### 引入可运行文件

```markdown
{{#playground demo/src/bin/example.rs}}
```

允许读者编辑：

```markdown
{{#playground demo/src/bin/example.rs editable}}
```

`{{#playground ...}}` 中的文件路径相对于当前 Markdown 文件。

传递给文件名后的值会成为 Rust 代码块属性：

```markdown
{{#playground demo/src/bin/example.rs editable edition2024}}
```

### 引入文件但不提供运行按钮

````markdown
```rust,no_run,noplayground
{{#include demo/src/bin/example.rs}}
```
````

### 只引入部分代码

按行号引入：

```markdown
{{#include demo/src/bin/example.rs:2:10}}
```

按 Anchor 引入更稳定，不会因为前面增加代码导致行号失效。

Rust 文件：

```rust
// ANCHOR: calculate
fn calculate(left: i32, right: i32) -> i32 {
    left + right
}
// ANCHOR_END: calculate
```

Markdown：

````markdown
```rust
{{#include demo/src/bin/example.rs:calculate}}
```
````

### 使用 `rustdoc_include`

`rustdoc_include` 会引入完整示例供编译测试，但初始只显示指定部分：

````markdown
```rust
{{#rustdoc_include demo/src/bin/example.rs:calculate}}
```
````

它适合讲解完整程序中的一个片段，同时保持示例可编译。

## 本项目推荐写法

对于需要运行或调试的示例，推荐：

```text
知识点/
├── README.md
└── demo/
    ├── Cargo.toml
    └── src/
        └── bin/
            └── example.rs
```

在 `README.md` 中引用：

```markdown
{{#playground demo/src/bin/example.rs editable}}
```

本地验证：

```powershell
cargo run -p package_name --bin example
mdbook test
mdbook build
```

### 当前项目的全局 Playground 配置

当前项目在根目录 `book.toml` 中启用了以下配置：

```toml
[rust]
edition = "2024"

[output.html.playground]
editable = true
copyable = true
copy-js = true
line-numbers = true
runnable = true
```

这些配置表示：

- Rust 示例默认使用 Edition 2024。
- Playground 代码块默认可以编辑和运行。
- 显示复制按钮与行号。
- 为代码复制功能加载 JavaScript。

全局启用 `editable` 后，仍然可以使用 `noplayground`、`no_run` 或 `ignore` 控制单个代码块的行为。

### 推荐选择规则

| 示例类型 | 推荐方式 |
| --- | --- |
| 简短、独立、可运行的语法示例 | `rust` 代码块 |
| 需要读者修改并尝试 | `rust,editable` |
| 工程中真实存在的可运行 Demo | `{{#playground ... editable}}` |
| 只展示真实文件中的一个片段 | `{{#include ...:anchor}}` |
| 展示片段但需要完整代码参与测试 | `{{#rustdoc_include ...:anchor}}` |
| 只需编译，不应运行 | `no_run` |
| 故意演示编译错误 | `rust,editable,ignore,mdbook-runnable` 内联代码块 |
| 伪代码或不完整代码 | 标记为 `text`，不要冒充可运行 Rust |

## 常见问题

### Playground 可以使用任意 Crate 吗？

不可以。在线 Playground 预装了一部分常用 Crate 及其依赖，但不能自由访问网络下载任意依赖。复杂依赖示例应放入本地 Cargo Demo。

### Playground 可以读取用户输入吗？

页面中的运行环境不适合交互式输入。依赖 `stdin` 等待输入的示例可能无法正常完成，应改为固定测试数据，或者使用 `noplayground`。

### Playground 可以用于性能测试吗？

不建议。Playground 运行在共享且受限的容器环境中，结果不能代表真实机器上的稳定性能。

### 如何展示编译错误？

本项目使用 `rust,editable,ignore,mdbook-runnable` 内联代码块。读者可以在页面中点击运行查看编译错误，而该代码块不会阻塞 `mdbook test`。

### 为什么 mdBook 页面没有运行按钮？

常见原因：

- 代码块没有标记为 `rust`。
- 使用了 `no_run`、`noplayground`，或者没有配合 `mdbook-runnable` 使用 `ignore`。
- `book.toml` 中全局关闭了 Playground。
- 示例依赖当前页面无法提供的外部环境。

## 相关资料

- [Rust Playground](https://play.rust-lang.org/)
- [Rust Playground 官方仓库](https://github.com/rust-lang/rust-playground)
- [mdBook：Rust Playground 与代码块属性](https://rust-lang.github.io/mdBook/format/mdbook.html#rust-playground)
- [mdBook：插入可运行 Rust 文件](https://rust-lang.github.io/mdBook/format/mdbook.html#inserting-runnable-rust-files)
- [rustdoc：Documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
