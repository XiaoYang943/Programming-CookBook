# 避免使用反射

> Effective Rust **Item 19**：[Avoid reflection](https://effective-rust.com/reflection.html)。

## 使用 trait 表达需要的能力

与其在运行时查询类型并分支，不如定义调用方真正需要的行为。

{{#playground demo/src/bin/trait_使用行为代替类型查询.rs editable}}

## 在受控边界使用 `Any`

`Any` 可以进行运行时类型检查，但会失去大量静态类型信息，只适合插件注册表、测试工具等受控边界。

{{#playground demo/src/bin/any_在受控边界查询类型.rs editable}}

## 选择原则

- 业务逻辑优先使用 trait、enum 和泛型。
- 类型集合封闭时使用 enum 穷尽匹配。
- 只有框架边界确实无法预知类型时才考虑 `Any`。
