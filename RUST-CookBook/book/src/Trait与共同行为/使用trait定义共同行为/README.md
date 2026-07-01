# 使用 trait 定义共同行为

> 《Rust 程序设计语言》：[使用 trait 定义共同行为](https://doc.rust-lang.org/book/ch10-02-traits.html)。

> trait 用来描述“某个类型具备什么能力”，`impl` 用来声明“某个具体类型具备这种能力”。

## trait 定义共同行为

- 先定义一种能力，再让多个 `struct` 分别实现它。

- 真实工程参考： geo `Area` trait：<https://github.com/georust/geo/blob/main/geo/src/algorithm/area.rs>
  - 多个几何类型都可以计算面积，但计算方式不同。抽象成 Rust 概念就是：**不同类型共享同一种行为名称，但各自拥有自己的实现**。

{{#playground demo/src/bin/trait_定义共同行为.rs editable}}

## type 关键字

### type 的用法
#### 用 type 定义类型别名

> type: 定义一个类型别名

```rust
type UserId = u64;
type ParseResult = Result<i32, String>;
```

#### 在 trait 中用 type 声明关联类型

> associated type(关联类型): 就是定义在 trait 里的 `type`，它表示“这个 trait 的实现者需要补充说明的相关类型”。

```rust
// 实现 `Parser` 的类型，必须告诉编译器自己的 `Output` 到底是什么类型。
trait Parser {
    type Output;    // 声明关联类型
}
```

比如：

```rust
struct NumberParser;

impl Parser for NumberParser {
    type Output = i32;  // 为该实现指定关联类型
}
```

### 关联类型的例子

有些 trait 不只需要方法，还需要表达“和实现者相关的类型”。最直观的例子是解析器：大家都叫 `parse`，输入都可以是文本，但解析结果可能是整数、布尔值或单词列表。

真实工程参考：

- geo `BoundingRect` trait：<https://github.com/georust/geo/blob/main/geo/src/algorithm/bounding_rect.rs>

`geo` 的 `BoundingRect` 用关联类型 `type Output` 让实现者决定返回值形态。下面的 demo 不直接使用几何概念，而是用更容易理解的 `Parser` 说明同一个 trait 方法如何返回不同类型。

{{#playground demo/src/bin/associated_type_关联类型.rs editable}}

### 关联类型和泛型参数对比

`type Output` 的优点在于：**输出类型属于实现者本身**。

也就是说，`NumberParser` 这个类型一旦实现了 `Parser`，它的输出就固定是 `i32`；`BoolParser` 的输出固定是 `bool`。调用方不需要在每次调用时再说明“我要解析成什么类型”。

这不是说泛型参数做不到，而是两者表达的语义不一样。

关联类型版本问的是：

> 你这个实现自己的输出类型是什么？

泛型参数版本问的是：

> 你要我针对哪个目标类型 `T` 实现能力？

如果不用关联类型，也可以把输出类型写成 trait 的泛型参数。下面这个对照 demo 是纯泛型版本，里面不会出现 `type Output`：

```rust
trait Parser<T> {
    fn parse(&self, input: &str) -> T;
}
```

这表示：`Parser<T>` 是“针对某个目标类型 T 的解析能力”。它更灵活，同一个解析器类型可以同时实现 `Parser<i32>`、`Parser<bool>`、`Parser<Vec<String>>`，但代价是调用处有时需要通过类型标注或完全限定语法说明到底要用哪一个实现。

换句话说，泛型参数版本没有错。它适合“同一个类型确实要支持多个目标输出”的场景。

但是如果一个类型天然只有一种输出，用关联类型会更清楚：

```rust
struct NumberParser;

impl Parser for NumberParser {
    type Output = i32;
}
```

这段代码表达的是：

> `NumberParser` 的身份就是“解析出 i32 的解析器”。

标准库里很多核心 trait 也采用这种设计，例如：

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

`Iterator::Item` 用关联类型，是因为一个具体迭代器产出的元素类型通常是它自身的一部分，而不是调用方每次临时选择的东西。

类似的还有：

- `Future::Output`：这个 future 完成后产出什么。
- `Deref::Target`：这个类型解引用后指向什么。
- `Index::Output`：这个类型索引后得到什么。

简单判断：

- 一种实现只有一种自然输出：优先用关联类型。
- 同一个实现需要支持多种目标输出：可以考虑泛型参数。

{{#playground demo/src/bin/associated_type_对比泛型参数.rs editable}}

## 标准Trait-Fn闭包参数

这一节不要先急着看 `Fn`。先看 `map_coords` 想解决什么问题。

真实工程参考：

- geo `MapCoords` trait：<https://github.com/georust/geo/blob/main/geo/src/algorithm/map_coords.rs>

### map_coords 要表达什么语义

`map_coords` 的核心语义是：

> 给一个结构里的每个坐标点应用同一种转换规则，并返回转换后的新结构。

比如一个点：

```rust
Point { x: 1.0, y: 2.0 }
```

可以被转换成：

- 平移后的点：`x + dx`，`y + dy`
- 缩放后的点：`x * factor`，`y * factor`
- 取整后的点：`x.round()`，`y.round()`
- 限制范围后的点：`x.clamp(min, max)`，`y.clamp(min, max)`

对于一个 `Line`：

```rust
Line {
    start: Point { ... },
    end: Point { ... },
}
```

语义就是：

> 对 `start` 和 `end` 都应用同一个点转换规则。

所以这里真正的问题不是“怎么写闭包”，而是：

> 如何让 `Line` 负责遍历自己的坐标结构，同时让调用方决定每个坐标怎么变？

### 常规思路：不用闭包时给每种转换写一个方法

如果先不用闭包，最直接的写法是给 `Line` 写很多专门方法：

```rust
line.translate(10.0, 20.0);
line.scale(2.0);
line.clamp(-180.0, 180.0);
```

这个思路很好理解：要平移就写 `translate`，要缩放就写 `scale`，要限制范围就写 `clamp`。

{{#playground demo/src/bin/fn_trait_不使用闭包.rs editable}}

### 常规思路的问题

上面的写法能工作，但它会带来几个问题。

第一，每新增一种转换规则，`Line` 的 API 就要新增一个方法。

```rust
fn translate(self, dx: f64, dy: f64) -> Self
fn scale(self, factor: f64) -> Self
fn clamp(self, min: f64, max: f64) -> Self
fn round(self) -> Self
fn flip_x(self) -> Self
```

第二，每个方法内部都会重复同一件事：

```rust
Self {
    start: 某种转换(self.start),
    end: 某种转换(self.end),
}
```

第三，`Line` 同时承担了两个职责：

- 它要知道自己的结构，也就是内部有 `start` 和 `end`。
- 它还要知道所有可能的坐标转换算法。

这会让接口越来越胖，也让重复代码越来越多。

### 解决方案：把变化规则作为参数传进去

既然固定的是“遍历结构”，变化的是“每个点怎么变”，那就可以把变化规则提出来，作为参数传给 `Line`：

```rust
trait MapCoords {
    fn map_coords(self, f: impl Fn(Point) -> Point) -> Self;
}
```

这段签名可以读成：

> `map_coords` 接收一段坐标转换规则，这段规则能把一个 `Point` 变成另一个 `Point`。

这样一来：

- `Line` 只负责遍历 `start` 和 `end`。
- 调用方只负责提供“点怎么变”的规则。

比如：

```rust
line.map_coords(|point| Point {
    x: point.x + 10.0,
    y: point.y + 20.0,
});
```

这里的 `|point| ...` 就是一段可以传进去的规则。

{{#playground demo/src/bin/fn_trait_闭包参数.rs editable}}

### 闭包是什么

闭包是一种可以保存到变量里、可以作为参数传递、也可以直接调用的匿名函数。

最简单的闭包长这样：

```rust
let add_one = |value| value + 1;

let result = add_one(41);
```

其中：

- `|value|` 是闭包的参数列表。
- `value + 1` 是闭包体。
- `add_one(41)` 表示像调用函数一样调用这个闭包。

闭包和普通函数最重要的区别是：**闭包可以捕获它所在作用域里的变量**。

```rust
let dx = 10.0;
let dy = 20.0;

let translate = |point: Point| Point {
    x: point.x + dx,
    y: point.y + dy,
};
```

这里的 `translate` 表面上是一个 `Point -> Point` 的规则，但它内部带着 `dx` 和 `dy`。

这就是闭包捕获环境的价值：

> 把“行为”和“这段行为需要用到的上下文数据”打包在一起传递。

如果不用闭包捕获环境，`dx`、`dy`、`factor`、`min`、`max` 这些上下文数据就很容易被塞进各种专门方法里，导致 API 不断膨胀。

### 为什么参数类型要写成 Fn

`Fn` 是标准库里的 trait，不是关键字。`impl Fn(...) -> ...` 是一种 trait bound 写法，用来约束“这个参数必须是可以被调用的东西”。

Rust 里的闭包、函数名、函数指针，都可以被当成“可调用对象”。为了描述这些可调用对象，标准库提供了三个核心 trait：

- `Fn`：可以被反复调用，并且不会修改捕获的外部变量。
- `FnMut`：可以被反复调用，但可能会修改捕获的外部变量。
- `FnOnce`：至少可以被调用一次；调用时可能会消耗自己捕获的值。

所以：

```rust
fn map_coords(self, f: impl Fn(Point) -> Point) -> Self
```

可以拆开读：

- `impl Trait`：参数位置的简写，表示“接收某个实现了这个 trait 的具体类型”。
- `Fn(Point) -> Point`：标准库闭包 trait，表示“能被调用，参数是 `Point`，返回值是 `Point`”。

也就是说：

> `f` 的具体类型叫什么不重要，只要它能被调用，并且满足“输入 `Point`，返回 `Point`”。

Rust 需要这种写法，是因为每个闭包都有编译器生成的匿名类型，普通代码写不出这个具体类型名。比如下面两个闭包，即使长得很像，也是不同的具体类型：

```rust
let add_one = |value| value + 1;
let add_two = |value| value + 2;
```

函数参数不能写成“某个说不出名字的闭包类型”，所以用 `Fn` trait 来描述它的能力。

### geo MapCoords 的真实设计

`geo` 的 `MapCoords` 大意是：

```rust
fn map_coords(
    &self,
    func: impl Fn(Coord<T>) -> Coord<NT> + Copy,
) -> Self::Output
```

这段签名表达的是：

> 几何类型自己负责遍历内部坐标，调用方传入一个闭包 `func`，决定每个坐标如何转换。

真实工程里的几何类型可能是 `Point`、`LineString`、`Polygon`、`MultiPolygon`。调用方不应该手动拆每一层结构；几何类型自己知道怎么遍历内部坐标，调用方只提供坐标转换规则。

### 小结：闭包在这里解决了什么问题

在这个例子里，闭包不是为了炫技。它解决的是 API 设计问题：

- `Line` 只负责结构遍历。
- 转换规则由调用方提供。
- 一个 `map_coords` 可以支持无数种坐标转换。
- 闭包可以捕获上下文，比如 `dx`、`dy`、`factor`、`min`、`max`。
- API 不会随着转换规则增加而不断膨胀。

一句话：

> 闭包把“带上下文的处理规则”包装成一个简单的 `Point -> Point` 形状传进去。

## blanket impl 条件实现

当一个类型已经具备某种基础能力时，可以通过 blanket impl 自动获得更高层能力。

真实工程参考：

- geo `AffineOps` blanket impl：<https://github.com/georust/geo/blob/main/geo/src/algorithm/affine_ops.rs>

`geo` 的设计大意是：只要一个类型会“映射坐标”，它就能自动获得平移、缩放、旋转等操作。抽象成 Rust 写法就是：

```rust
impl<T> SomeTrait for T
where
    T: OtherTrait,
{
}
```

这不是为了少写代码而已，而是为了表达一种能力推导关系：**会 A 的类型，自动会 B**。

{{#playground demo/src/bin/blanket_impl_条件实现.rs editable}}

## impl Trait 简化签名

参数位置的 `impl Trait` 是泛型参数的简写；返回位置用于隐藏具体返回类型，但单个返回路径必须保持同一具体类型。

{{#playground demo/src/bin/impl_trait_简化参数与返回类型.rs editable}}

{{#playground snippets/impl_trait_不能返回不同具体类型.rs editable ignore mdbook-runnable}}

## 孤儿规则

只有 trait 或被实现类型至少有一个定义在当前 crate 中时，才能实现该 trait。

这个规则保证了不同 crate 不会同时给同一组“外部 trait + 外部类型”添加冲突实现。
