# 使用临时可变性明确修改范围

> 本文整理自 Rust Design Patterns 的 Idiom：[Temporary mutability](https://rust-unofficial.github.io/patterns/idioms/temporary-mutability.html)。

## 使用代码块限制 `mut` 作用域

**核心规则**

只在构建或修改数据的短暂阶段使用 `mut`，离开 Block 后将结果绑定为不可变值。

{{#playground demo/src/bin/block_限制mut作用域.rs editable}}

## 使用变量遮蔽恢复不可变绑定

**核心规则**

完成修改后，可以使用 Shadowing 创建同名的不可变绑定，明确后续代码不再修改该值。

{{#playground demo/src/bin/shadowing_恢复不可变绑定.rs editable}}

## 选择临时可变方式

| 场景 | 选择 |
| --- | --- |
| 修改过程包含多个步骤或辅助变量 | 使用 Block 限制作用域 |
| 修改后仍希望复用原变量名称 | 使用 Shadowing |
| 整个函数中状态确实持续变化 | 保留 `mut`，但尽量缩小作用域 |

## 实践建议

- 让 `mut` 的有效范围尽可能短。
- 在修改结束后，让类型和绑定重新表达不可变意图。
- 不要为了避免 `mut` 写出更难理解的代码；清晰度优先。
