# 闭包

> 《Rust 程序设计语言》：[闭包](https://doc.rust-lang.org/book/ch13-01-closures.html)。

> 《Rust Reference》：[闭包表达式](https://doc.rust-lang.org/reference/expressions/closure-expr.html)。

> 《Rust Reference》：[闭包类型](https://doc.rust-lang.org/reference/types/closure.html)。

## 闭包是什么

> 闭包是一种可以被当作值保存和传递的**匿名函数**

普通函数通常需要先起名字：

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

闭包可以直接写在使用的地方：

```rust
let add_one = |x: i32| x + 1;

println!("{}", add_one(5)); // 6
```

这里的 `|x: i32| x + 1` 就是一个闭包。它没有函数名，但可以被保存到变量 `add_one` 中，之后像函数一样调用。

## 闭包的特点
### 可以使用围作用域中的变量

```rust
let base = 10;

let add_base = |x: i32| x + base;

println!("{}", add_base(5)); // 15
```

## 闭包的作用

当一段逻辑很短，只在某个地方临时使用时，闭包比单独定义一个函数更方便。更重要的是：闭包可以直接使用当前作用域里的局部变量，所以很适合表达“这一次调用要怎么处理”。

下面每个场景都给出“使用闭包”和“不使用闭包”的对照。它们都能完成任务，但闭包版通常能把规则直接写在调用处。

### 迭代器 + 闭包：定义map规则

`map` 接收一个闭包，用来描述“每个元素要怎么变”。闭包的优势不只是写法短，而是它可以把当前作用域中的变量带进迭代器链。

先看一个只依赖固定参数 `factor` 的例子。因为 `factor` 每次迭代都一样，所以不使用闭包时，还可以用 `zip(std::iter::repeat(factor))` 把它塞进每个迭代项里。

使用闭包：

{{#playground demo/src/bin/closure_作用_迭代器.rs editable}}

不使用闭包：

{{#playground demo/src/bin/closure_作用_迭代器_不使用闭包.rs editable}}

再看一个同时依赖 `sum` 和 `factor` 的例子。`factor` 是固定参数，但 `sum` 是每次迭代都会变化的状态，不能提前用 `repeat` 或 `zip` 准备好。

使用闭包：

{{#playground demo/src/bin/closure_作用_迭代器_sum_factor_使用闭包.rs editable}}

不使用闭包：

{{#playground demo/src/bin/closure_作用_迭代器_sum_factor_不使用闭包.rs editable}}

### 排序 + 闭包：定义排序规则

`sort_by_key` 接收一个闭包，用来描述“从每个元素里取出哪个值作为排序依据”

使用闭包：

{{#playground demo/src/bin/closure_作用_排序.rs editable}}

不使用闭包：

{{#playground demo/src/bin/closure_作用_排序_不使用闭包.rs editable}}

### 迭代器 + 闭包：定义filter规则

`filter` 接收一个闭包，用来描述“什么样的元素应该被保留”。下面的 `|n| *n > min && n % 2 == 0` 表示只保留大于 `min` 的偶数。

使用闭包：

{{#playground demo/src/bin/closure_作用_筛选.rs editable}}

不使用闭包：

{{#playground demo/src/bin/closure_作用_筛选_不使用闭包.rs editable}}

对比可以看到：不用闭包时，需要拆出判断函数，再手动循环收集结果；闭包版可以直接在 `filter` 中捕获 `min`。

### 延迟执行 + 闭包：定义备用逻辑

`Result::unwrap_or_else` 接收一个闭包，用来描述“出错时才执行的备用逻辑”。如果结果是 `Ok(value)`，直接返回 `value`；如果结果是 `Err(error)`，才执行闭包，并把 `error` 传给闭包。

使用闭包：

{{#playground demo/src/bin/closure_作用_延迟执行.rs editable}}

不使用闭包：

{{#playground demo/src/bin/closure_作用_延迟执行_不使用闭包.rs editable}}

对比可以看到：不用闭包时，需要自己写 `match` 区分 `Ok` 和 `Err`；闭包版可以把“出错时才怎么处理错误”直接交给 `unwrap_or_else`。
## 使用闭包保存行为

闭包可以把“一段稍后执行的逻辑”保存到变量中。这个变量保存的不是某一次计算结果，而是一条计算规则。

下面的 `multiply` 保存了“把数字乘以 `factor`”这个行为。`apply_to_all` 不关心具体规则是什么，只负责把传进来的行为应用到每个数字上。

{{#playground demo/src/bin/closure_定义并调用闭包.rs editable}}

这个例子中，闭包的优势是：调用方可以临时决定要传入什么行为，并且这个行为还能直接使用当前作用域中的 `factor`。
## 捕获不可变引用与可变引用

编译器会根据闭包体的实际需要，选择不可变借用、可变借用或取得所有权。

{{#playground demo/src/bin/closure_捕获不可变与可变引用.rs editable}}

## 使用 `move` 取得环境值的所有权

`move` 强制闭包取得所使用环境值的所有权，常用于线程或需要让闭包离开当前作用域的场景。

{{#playground demo/src/bin/move_取得环境值所有权.rs editable}}

## 使用 `Fn`、`FnMut` 与 `FnOnce` 约束闭包

| trait | 闭包对捕获值的使用方式 | 调用次数 |
| --- | --- | --- |
| `Fn` | 只读借用 | 可多次 |
| `FnMut` | 可变借用 | 可多次 |
| `FnOnce` | 可能消费捕获值 | 至少可调用一次 |

每个闭包至少实现 `FnOnce`；没有消费捕获值时，还可能实现 `FnMut` 或 `Fn`。

{{#playground demo/src/bin/fn_traits_约束闭包行为.rs editable}}

## 理解闭包类型推断

闭包第一次调用后，参数与返回值的具体类型就会固定，不能再用另一种参数类型调用同一个闭包。

{{#playground snippets/closure_参数类型推断后固定.rs editable ignore mdbook-runnable}}

