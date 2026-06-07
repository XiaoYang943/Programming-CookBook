# 变量与可变性

> 《Rust 程序设计语言》原文：[变量与可变性](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)。

> 《Rust Reference》：[变量](https://doc.rust-lang.org/reference/variables.html)。

> 《Rust Reference》：[下划线表达式](https://doc.rust-lang.org/reference/expressions/underscore-expr.html)。

## 使用 `let` 声明不可变绑定

**核心规则**

`let` 创建名称与值之间的绑定。绑定默认不可变，声明后不能通过该绑定重新赋值。

**基本示例**

{{#playground demo/src/bin/immutable_不可变绑定.rs editable}}

**语法限制**

尝试重新赋值时，编译器会拒绝代码：

{{#playground snippets/immutable_不可变绑定不能重新赋值.rs editable ignore mdbook-runnable}}

不可变限制的是通过绑定重新赋值，不代表值内部在任何情况下都不能变化；内部可变性将在智能指针相关章节讨论。

## 使用 `mut` 重新赋值

**核心规则**

在变量名称前添加 `mut`，允许通过该绑定重新赋值，并明确表达值预计会发生变化。

**基本示例**

{{#playground demo/src/bin/mutable_可变绑定.rs editable}}

**适用范围**

函数参数也是默认不可变的变量，可以添加 `mut` 后在函数体内重新赋值。

{{#playground demo/src/bin/mutable_parameter_可变函数参数.rs editable}}

**语法限制**

`mut` 允许重新赋值，但不能改变绑定的类型。

{{#playground snippets/mutable_重新赋值不能改变类型.rs editable ignore mdbook-runnable}}

## 使用 `const` 声明常量

**核心规则**

`const` 声明编译期常量，必须标注类型并使用常量表达式初始化。

**核心区别**

| 对比项 | `let` 不可变绑定 | `const` 常量 |
| --- | --- | --- |
| 声明关键字 | `let` | `const` |
| 能否添加 `mut` | 可以 | 不可以 |
| 是否必须标注类型 | 不必须 | 必须 |
| 值是否可以在运行时计算 | 可以 | 必须是常量表达式 |
| 能否定义在局部作用域外 | 不可以 | 可以 |
| 常用命名风格 | `snake_case` | `SCREAMING_SNAKE_CASE` |

{{#playground demo/src/bin/constant_常量.rs editable}}

**选择原则**

当值需要共享、能在编译期确定并具有明确领域含义时，使用 `const`。

## 使用变量遮蔽创建新绑定

**核心规则**

使用同一个名称再次执行 `let`，会创建新绑定并遮蔽(shadow)旧绑定。Shadowing 不是修改原变量。

**基本示例**

{{#playground demo/src/bin/shadowing_变量遮蔽.rs editable}}

**适用范围**

Shadowing 创建新绑定，因此可以在复用名称时改变值的类型。

{{#playground demo/src/bin/shadowing_改变类型.rs editable}}

## 对比 `mut` 与变量遮蔽

**核心区别**

| 对比项 | `mut` | Shadowing |
| --- | --- | --- |
| 是否创建新绑定 | 否 | 是 |
| 是否再次使用 `let` | 否 | 是 |
| 能否改变类型 | 不能 | 可以 |
| 后续是否可以保持不可变 | 否 | 可以 |
| 典型用途 | 同一个值持续变化 | 数据转换后重新解释 |

**选择原则**

- 同一个状态需要持续变化时，使用 `mut`。
- 数据经过解析、校验或转换后不再变化时，使用 Shadowing。

## 使用 `let` 延迟初始化

**核心规则**

`let` 可以先声明、稍后初始化。第一次赋值属于初始化，而不是重新赋值。

**基本示例**

{{#playground demo/src/bin/delayed_initialization_延迟初始化.rs editable}}

**语法限制**

编译器会检查所有控制流路径；只要某条路径可能没有初始化变量，就会拒绝使用它。

{{#playground snippets/delayed_initialization_所有路径必须初始化.rs editable ignore mdbook-runnable}}

**实践建议**

优先让声明靠近初始化位置，仅在需要避免无意义默认值时使用延迟初始化。

## 使用 `_` 丢弃解构赋值中的值

**核心规则**

下划线表达式 `_` 可以出现在赋值语句左侧。它会丢弃对应位置的值，不创建变量绑定。

右侧表达式仍然会被完整计算，只是其中与 `_` 对应的值不会赋给任何变量。

**基本示例**

在解构赋值中，可以使用 `_` 丢弃不需要的部分，只给已有变量赋值。

{{#playground demo/src/bin/underscore_下划线表达式.rs editable}}

**语法限制**

下划线表达式 `_` 不能作为普通值使用。将它放在赋值语句左侧以外的位置时，编译器会报错。

{{#playground snippets/underscore_下划线不能作为值.rs editable ignore mdbook-runnable}}

**易混淆语法**

以下三种写法外观相似，但属于不同语法结构：

| 写法 | `_` 的含义 | 是否创建变量 | 主要用途 |
| --- | --- | --- | --- |
| `_ = value` | 下划线表达式 | 否 | 丢弃赋值位置中的值 |
| `let _ = value` | 通配符模式 | 否 | 明确忽略整个值 |
| `let _unused = value` | 变量名的一部分 | 是 | 保留值，但避免未使用变量警告 |

{{#playground demo/src/bin/underscore_三种忽略方式.rs editable}}

**资源生命周期**

`_ = value` 和 `let _ = value` 都不会创建绑定，产生的值会立即被丢弃。`let _name = value` 会创建真实绑定，让值存活到该绑定离开作用域。

这对锁 Guard、文件句柄等具有 `Drop` 行为的资源尤其重要：需要持续持有资源时，必须创建真实绑定。

{{#playground demo/src/bin/underscore_绑定影响资源生命周期.rs editable}}

## `let`、`mut` 与 `const` 实践

**默认选择**

默认使用不可变绑定，仅在确实需要重新赋值时添加 `mut`。

**选择建议**

- 默认保持绑定不可变，只在确实需要重新赋值时添加 `mut`。
- 使用 `mut` 表达持续变化的状态，使用 Shadowing 表达数据转换。
- 尽量缩小可变绑定的作用域。
- 常量使用明确的类型和 `SCREAMING_SNAKE_CASE` 名称。
- 让变量声明靠近初始化位置；需要延迟初始化时，确保所有控制流路径都完成初始化。
- 使用 `_` 明确丢弃不需要的值；需要保持资源存活时，使用真实绑定。
