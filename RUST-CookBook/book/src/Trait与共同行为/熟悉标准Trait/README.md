# 熟悉标准 trait

> Effective Rust **Item 10**：[Familiarize yourself with standard traits](https://effective-rust.com/std-traits.html)。

- 标准库 trait 是学习 trait 非常重要的一步。Rust 把很多语言行为和生态约定都编码为
细粒度 trait：能否复制、能否比较、能否哈希、能否格式化、能否迭代、能否跨线程
传递，通常都由标准 trait 表达。

- 因此，看一个类型实现了哪些标准 trait，基本就能判断它可以参与哪些 Rust 语法和
库生态：

| trait | 让类型获得的能力 | 常见语法或 API |
| --- | --- | --- |
| `Clone` | 显式复制值 | `value.clone()` |
| `Copy` | 赋值和传参时按位复制 | `let y = x` 后 `x` 仍可用 |
| `Default` | 构造默认值 | `T::default()`、`Default::default()` |
| `PartialEq` / `Eq` | 判断相等 | `==`、`!=` |
| `PartialOrd` / `Ord` | 比较顺序 | `<`、`>`、`<=`、`>=`、排序 |
| `Hash` | 计算哈希 | `HashMap`、`HashSet` 的 key |
| `Debug` | 给开发者看的格式化输出 | `{:?}` |
| `Display` | 给用户看的格式化输出 | `{}`、`to_string()` |
| `From` / `TryFrom` | 类型转换 | `from`、`into`、`try_from` |
| `Iterator` | 按序产生元素 | `for`、`map`、`filter`、`sum` |

## 理解 `derive` 做了什么

`derive` 是让编译器根据类型字段自动生成某些 trait 实现的语法。

例如：

```rust
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct ReleaseVersion {
    major: u16,
    minor: u16,
    patch: u16,
}
```

这段代码不是给 `ReleaseVersion` 添加普通字段，而是让编译器自动生成类似下面的 trait
实现：

```rust
impl Clone for ReleaseVersion { /* 编译器生成 */ }
impl Copy for ReleaseVersion { /* 编译器生成 */ }
impl Default for ReleaseVersion { /* 编译器生成 */ }
impl PartialEq for ReleaseVersion { /* 编译器生成 */ }
impl Eq for ReleaseVersion { /* 编译器生成 */ }
impl PartialOrd for ReleaseVersion { /* 编译器生成 */ }
impl Ord for ReleaseVersion { /* 编译器生成 */ }
impl Hash for ReleaseVersion { /* 编译器生成 */ }
impl Debug for ReleaseVersion { /* 编译器生成 */ }
```

生成这些实现后，`ReleaseVersion` 才具备对应能力：

- 有 `Default`，才能调用 `ReleaseVersion::default()`。
- 有 `Clone`，才能调用 `version.clone()`。
- 有 `Copy`，赋值和传参时才不会移动所有权。
- 有 `PartialEq`，才能使用 `==`、`!=`、`assert_eq!`、`assert_ne!`。
- 有 `Eq` 和 `Hash`，才能可靠地作为 `HashMap` 的 key。
- 有 `PartialOrd` 和 `Ord`，才能比较大小、排序或放入 `BTreeSet`。
- 有 `Debug`，才能使用 `{:?}` 输出调试信息。

`Display` 也属于常见标准 trait，但它通常不能通过 `derive` 自动生成，因为面向用户的
文本格式需要开发者明确设计。

“能派生时优先使用 `derive`”的意思是：如果字段级的默认实现正好符合业务语义，就让
编译器生成实现，而不是手写重复代码。这样更短、更稳定，也更容易让读者看出这个类型
具备哪些标准能力。

但 `derive` 不是永远正确。派生实现会按字段逐个处理，如果字段级行为不符合业务语义，
就应该手动实现，或者不要实现这个 trait。例如，用户相等性只由 `user_id` 决定时，
就不应该盲目派生 `PartialEq` 让所有字段都参与比较。

Effective Rust 将 `Clone`、`Copy`、`Default`、`PartialEq`、`Eq`、`PartialOrd`、
`Ord`、`Hash`、`Debug`、`Display` 列为最常见的一组标准 trait。

下面的完整示例使用 `ReleaseVersion` 表示版本号。版本号天然适合复制、比较、排序、
作为映射 key、调试输出和用户展示，因此适合放在同一个 demo 中观察这十个 trait 如何
协同工作。

{{#playground demo/src/bin/derive_实现常用标准trait.rs editable}}

## 用 `Clone` 显式复制一份值

`Clone` 表示“复制可能有成本，所以调用者要显式写出复制动作”。它适合 `String`、
`Vec<T>`、配置对象、请求参数等需要保留原值并创建新值的场景。

### 最佳实践：需要复制堆数据时使用

堆数据通常不能像整数一样按位复制，因为按位复制会导致多个值指向同一份堆内存并重复
释放。`Clone` 让类型自己定义如何复制内部资源，例如为 `String` 分配一份新的堆内存。

```rust
#[derive(Clone)]
struct SearchRequest {
    keyword: String,
    page_size: u32,
}

let request = SearchRequest {
    keyword: String::from("rust"),
    page_size: 20,
};

// clone 明确告诉读者：这里复制了一份 String 背后的堆数据
let retry_request = request.clone();
```

### 最佳实践：复制行为有业务含义时显式写出

有些复制不是性能问题，而是语义问题。例如重试请求时保留原请求参数，使用 `clone`
能让“基于原请求再发一次”的意图更清楚。

```rust
#[derive(Clone)]
struct ExportJob {
    report_name: String,
}

fn enqueue_retry(job: &ExportJob) -> ExportJob {
    // 这里 clone 表达：重试任务复用原任务参数，但产生一个新的任务值
    job.clone()
}
```

### 常见误区：不要用 `Clone` 逃避所有权设计

遇到 borrow checker 错误就立刻 `.clone()`，可能会掩盖真正的问题：函数是否应该借用
参数，而不是取得所有权。无意义的复制会增加内存分配，也会让 API 语义变模糊。

```rust
fn print_keyword(keyword: &str) {
    // 只读取字符串时借用即可，不需要调用方 clone 出新的 String
    println!("{keyword}");
}
```

### 常见误区：不要在热路径中无意识复制大对象

`Clone` 可能是昂贵操作。`Vec<T>`、`HashMap<K, V>`、大字符串的 clone 都可能复制大量
数据。循环、请求处理、序列化前后这些热路径中，应该优先确认是否可以借用。

```rust
fn total_len(values: &[String]) -> usize {
    // 这里只需要读取字符串长度，借用切片比克隆 Vec<String> 更合适
    values.iter().map(String::len).sum()
}

// 不推荐：为了遍历而 clone 整个 Vec<String>
// let copied = values.clone();
```

## 用 `Copy` 表达廉价的隐式复制

`Copy` 表示赋值和传参时可以自动按位复制，原值仍然可用。它适合整数、布尔值、字符、
小型 ID、坐标、版本号等纯值语义类型。

### 最佳实践：只给小型纯值类型实现 `Copy`

小型纯值类型没有独占资源所有权，复制一个新值不会改变程序语义。给这类类型实现
`Copy`，可以让 API 使用起来像内置数字类型一样自然。

```rust
#[derive(Clone, Copy)]
struct UserId(u64);

let id = UserId(42);

// 赋值会复制 UserId，id 之后仍然可用
let copied = id;
let _still_available = id;
```

### 最佳实践：实现 `Copy` 时也必须实现 `Clone`

`Copy` 是 `Clone` 的子 trait。能 `Copy` 的类型一定能 `Clone`，并且 `clone` 通常只
返回 `*self`。使用 `derive(Clone, Copy)` 是最直接、最不容易出错的方式。

```rust
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 3, y: 4 };

// 对 Copy 类型来说，clone 和普通复制语义一致
let same_point = point.clone();
```

### 常见误区：持有资源所有权的类型不应实现 `Copy`

如果类型负责释放文件、网络连接、锁、堆内存等资源，隐式复制会制造多个“所有者”，这
与 Rust 的所有权模型冲突。标准库中的 `String`、`Vec<T>`、`File` 都不是 `Copy`。

```rust
struct Connection {
    id: u64,
}

// 如果 Connection 真实持有网络连接，就不应该实现 Copy
// 否则一次赋值可能让两个值看起来都拥有同一个连接
```

### 常见误区：不要把 `Copy` 当成性能优化开关

`Copy` 的重点是语义，不是“更快”。如果类型复制很便宜但语义上代表唯一资源，也不应
实现 `Copy`。反过来，如果语义上就是一个普通值，`Copy` 才合适。

```rust
#[derive(Clone)]
struct SessionToken(String);

// token 代表一段敏感凭证，即使内部只是字符串，也应避免无意复制
// 显式 Clone 能让复制凭证的动作更醒目
```

## 用 `Default` 表达有意义的默认值

`Default` 表示类型存在一个合理、可解释的默认值。默认值不是“随便填一个能编译的值”，
而应该是调用方可以安全理解和使用的初始状态。

### 最佳实践：默认值应代表安全的初始状态

配置类型、选项类型、计数器类型常常适合实现 `Default`，因为它们存在明确的空状态或
保守状态。默认值越明确，调用方越容易使用 struct 更新语法覆盖少数字段。

```rust
#[derive(Default)]
struct RetryPolicy {
    max_retries: u32,
    backoff_millis: u64,
}

let policy = RetryPolicy {
    max_retries: 3,
    // 其他字段使用默认值，调用方只关注自己要改的部分
    ..RetryPolicy::default()
};
```

### 最佳实践：领域默认值不等于字段类型默认值时手动实现

派生 `Default` 会把数字设为 `0`、布尔设为 `false`、字符串设为空字符串。如果业务
默认值不是这些字段默认值，就应该手写实现。

```rust
struct PageOptions {
    page: u32,
    page_size: u32,
}

impl Default for PageOptions {
    fn default() -> Self {
        Self {
            page: 1,
            // 业务默认每页 20 条，而不是 u32 的默认值 0
            page_size: 20,
        }
    }
}
```

### 常见误区：不存在自然默认值时不要实现 `Default`

有些类型必须由外部提供有效值，例如邮箱、数据库连接 URL、认证令牌。给它们硬造默认
值会让无效状态进入系统，错误可能延迟到更远的位置才暴露。

```rust
struct EmailAddress(String);

// 不推荐为了方便而让 EmailAddress::default() 返回空字符串
// 空字符串不是有效邮箱，会把错误推迟到发送邮件时才出现
```

### 常见误区：不要把 `Default` 当作配置校验的替代品

默认值只能提供初始值，不能保证用户覆盖后的配置仍然合法。需要校验的配置仍应提供
构造函数或验证方法。

```rust
#[derive(Default)]
struct ServerConfig {
    port: u16,
}

impl ServerConfig {
    fn validate(&self) -> bool {
        // Default 得到的 0 端口未必符合业务要求，因此仍然需要显式校验
        self.port > 0
    }
}
```

## 用 `PartialEq` 表达业务相等

`PartialEq` 允许使用 `==` 和 `!=`。它回答的是“这两个值在当前业务语义下是否相等”，
而不一定是“所有字段是否完全相同”。

### 最佳实践：相等字段应表达业务身份

如果一个类型的业务身份由 ID 决定，那么缓存字段、展示字段、统计字段通常不应参与
相等判断。派生实现会比较所有字段，因此不符合时要手动实现。

```rust
struct UserSnapshot {
    user_id: u64,
    display_name: String,
    cached_score: u32,
}

impl PartialEq for UserSnapshot {
    fn eq(&self, other: &Self) -> bool {
        // 业务上同一个 user_id 就代表同一个用户快照身份
        self.user_id == other.user_id
    }
}
```

### 最佳实践：字段级相等符合语义时优先派生

对于版本号、坐标、金额分量这类所有字段共同决定身份的类型，派生 `PartialEq` 更清楚，
也更不容易漏字段。

```rust
#[derive(PartialEq)]
struct Money {
    cents: i64,
    currency: String,
}

// cents 和 currency 都参与相等判断，派生实现正好符合语义
```

### 常见误区：不要让临时状态影响业务相等

如果把缓存时间、命中次数、调试标记等临时字段纳入相等判断，集合去重、测试断言和业务
判断都可能产生意外结果。

```rust
struct CachedUser {
    user_id: u64,
    cache_hits: u64,
}

// 误区：如果派生 PartialEq，cache_hits 不同会导致同一用户被视为不相等
```

### 常见误区：不要让 `PartialEq` 与后续 `Hash`、`Ord` 语义冲突

如果相等只看 `user_id`，那么哈希和排序也应围绕同一身份设计。否则同一个值在
`HashMap`、排序列表、去重逻辑中的表现会不一致。

```rust
// 如果 PartialEq 只比较 user_id
// Hash 也必须只基于 user_id，否则 HashMap 查找会出现违反契约的问题
```

## 用 `Eq` 表达完整可靠的相等关系

`Eq` 是 marker trait，没有新增方法。它表示 `PartialEq` 的相等关系是完整的，尤其
满足自反性：任何值都应等于自身。

### 最佳实践：普通结构化数据实现 `PartialEq` 时通常也实现 `Eq`

整数、字符串、枚举、由这些字段组成的结构体通常具有完整相等关系。只要没有 `NaN`
这类特殊值，派生 `Eq` 能告诉集合类型和读者：这个相等判断可靠。

```rust
#[derive(PartialEq, Eq)]
struct OrderId(u64);

let id = OrderId(1001);

// Eq 承诺：值总是等于自身
assert!(id == id);
```

### 最佳实践：需要作为 `HashMap` key 时检查 `Eq`

`HashMap` 的 key 需要 `Eq + Hash`。这不是偶然限制：哈希表必须能可靠判断两个 key
是否相同，才能决定覆盖、查找或插入。

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct RequestId(u64);

let mut requests = HashMap::new();
requests.insert(RequestId(1), "created");
```

### 常见误区：浮点数不能直接实现 `Eq`

浮点数中的 `NaN` 不等于自身，所以 `f32`、`f64` 只实现 `PartialEq`，没有实现 `Eq`。
包含浮点字段的类型不能盲目派生 `Eq`。

```rust
struct Metric {
    value: f64,
}

// f64::NAN != f64::NAN，因此 Metric 不能简单承诺完整相等关系
```

### 常见误区：不要手写违反自反性的相等逻辑

如果 `x == x` 可能为 `false`，就不应实现 `Eq`。违反这个契约会让集合、去重和查找
逻辑失去可靠基础。

```rust
struct MaybeComparable {
    valid: bool,
}

// 如果 eq 在 valid 为 false 时返回 false，即使比较自身也不相等
// 这种类型最多适合 PartialEq，不适合 Eq
```

## 用 `PartialOrd` 表达可能不存在的排序

`PartialOrd` 允许比较大小，但比较结果可能不存在。它适合浮点数、包含浮点数的度量值、
或某些业务上并非任意两个值都能排序的类型。

### 最佳实践：比较可能失败时使用 `PartialOrd`

`partial_cmp` 返回 `Option<Ordering>`，用 `None` 表示无法比较。浮点数的 `NaN` 就是
典型例子。

```rust
let value = f64::NAN;

// NaN 与任何浮点数都无法确定大小关系
assert_eq!(value.partial_cmp(&1.0), None);
```

### 最佳实践：用比较运算符时明确无法比较的后果

`<`、`>` 等运算符在 `partial_cmp` 为 `None` 时会返回 `false`。这在过滤或排序前
需要特别注意，否则 `NaN` 可能悄悄绕过判断。

```rust
let value = f64::NAN;

// 两个判断都为 false，不代表 value 等于 1.0
assert!(!(value < 1.0));
assert!(!(value > 1.0));
```

### 常见误区：不要用 `PartialOrd` 强行表达全序

如果业务要求任意两个值都能排序，`PartialOrd` 的 `None` 会让逻辑多出一个难处理的
分支。此时应清洗数据、拒绝非法值，或使用能提供全序的包装类型。

```rust
fn is_valid_score(score: f64) -> bool {
    // 排序成绩前先拒绝 NaN，避免 PartialOrd 的不可比较状态进入排序逻辑
    !score.is_nan()
}
```

### 常见误区：派生排序会按字段声明顺序比较

派生 `PartialOrd` 会先比较第一个字段，再比较第二个字段。字段顺序如果不是业务顺序，
派生结果就会令人困惑。

```rust
#[derive(PartialEq, PartialOrd)]
struct VersionLike {
    minor: u16,
    major: u16,
}

// 误区：这里会先比较 minor，再比较 major
// 如果业务要按 major -> minor 排序，应调整字段顺序或手动实现
```

## 用 `Ord` 表达完整稳定的全序关系

`Ord` 表示任意两个值都能比较出确定顺序。它适合版本号、时间戳、整数 ID、优先级等
具有完整排序语义的类型。

### 最佳实践：需要排序或有序集合时实现 `Ord`

切片排序、`BTreeMap`、`BTreeSet` 都依赖稳定全序。实现 `Ord` 后，类型可以自然进入
这些 API。

```rust
use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);

let mut priorities = BTreeSet::new();
priorities.insert(Priority(10));
priorities.insert(Priority(1));
```

### 最佳实践：`Ord` 必须和 `Eq` 保持一致

如果 `a.cmp(&b) == Ordering::Equal`，那么 `a == b` 也应该为 `true`。否则排序去重、
有序集合插入等操作会出现难以解释的行为。

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ReleaseVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

// 派生实现会让 Eq 和 Ord 使用同一组字段，因此契约一致
```

### 常见误区：不要把显示顺序误当作业务排序

字符串显示格式和业务排序不一定一致。例如版本号按字符串排序时，`"1.10.0"` 会排在
`"1.2.0"` 前面或后面的结果可能不符合语义。应按结构化字段排序。

```rust
let versions = ["1.10.0", "1.2.0"];

// 误区：直接按字符串排序不是语义化版本排序
// 更好的做法是解析成 ReleaseVersion 后按 major minor patch 排序
```

### 常见误区：不要让可变字段影响有序集合中的顺序

放入 `BTreeSet` 或作为 `BTreeMap` key 后，如果影响排序的字段通过内部可变性发生变化，
集合内部顺序就可能失效。

```rust
// 有序集合的 key 应尽量是不可变值语义类型
// 不要把会变化的优先级对象直接作为 BTreeSet key
```

## 用 `Hash` 支持哈希集合与映射

`Hash` 让类型可以写入哈希器，常与 `Eq` 一起用于 `HashMap` 和 `HashSet`。

### 最佳实践：`Hash` 必须和 `Eq` 使用一致字段

哈希表的基本契约是：如果 `x == y`，那么二者的哈希结果也必须一致。最稳妥的方式是
同时派生 `PartialEq`、`Eq` 和 `Hash`，让它们使用同一组字段。

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct UserId(u64);

let mut names = HashMap::new();
names.insert(UserId(1), "Alice");
```

### 最佳实践：手写相等时重新审查哈希实现

只要手写了 `PartialEq`，就不能再机械派生 `Hash`。应确保参与相等的字段和参与哈希的
字段一致。

```rust
use std::hash::{Hash, Hasher};

struct User {
    id: u64,
    display_name: String,
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 如果 PartialEq 只按 id 判断相等，Hash 也应该只写入 id
        self.id.hash(state);
    }
}
```

### 常见误区：相等忽略字段，哈希却包含字段

如果 `PartialEq` 认为两个用户相等，但 `Hash` 因为显示名不同给出不同哈希值，
`HashMap` 查找可能失败或表现异常。

```rust
// 误区示意：
// eq 只比较 id
// hash 却同时写入 id 和 display_name
// 这违反了 HashMap 对 key 的基本要求
```

### 常见误区：不要把不稳定数据放进哈希身份

时间戳、缓存命中次数、调试标记等不稳定字段通常不应参与哈希身份，否则同一业务对象在
集合中会表现为多个 key。

```rust
struct CacheEntry {
    key: String,
    last_access_millis: u64,
}

// 通常 key 才是哈希身份，last_access_millis 只是运行时状态
```

## 用 `Debug` 输出开发者调试信息

`Debug` 面向开发者，通常用于日志、断言失败、测试输出和临时排查问题。它的格式不应
被当作稳定协议。

### 最佳实践：自定义类型通常应派生 `Debug`

派生 `Debug` 能显著改善调试体验。测试失败时，`assert_eq!` 也要求值实现 `Debug`
才能打印左右两边的内容。

```rust
#[derive(Debug, PartialEq)]
struct ApiStatus {
    code: u16,
    message: String,
}

// Debug 输出适合开发者查看字段和值
let text = format!("{:?}", ApiStatus { code: 200, message: String::from("ok") });
```

### 最佳实践：敏感信息类型要手写或避免输出敏感字段

令牌、密码、密钥等类型如果直接派生 `Debug`，可能在日志里泄露秘密。此时应手写
`Debug`，隐藏敏感字段。

```rust
use std::fmt;

struct SecretToken(String);

impl fmt::Debug for SecretToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 不把真实 token 写入日志
        formatter.write_str("SecretToken(<redacted>)")
    }
}
```

### 常见误区：不要把 `Debug` 当作用户可见输出

`Debug` 的输出格式服务于开发者，可能包含字段名、结构体名，也可能随着代码调整改变。
用户界面、文件格式、网络协议不应依赖它。

```rust
#[derive(Debug)]
struct InvoiceId(u64);

// 误区：把 "InvoiceId(42)" 当成用户要看的发票编号
let developer_text = format!("{:?}", InvoiceId(42));
```

### 常见误区：不要承诺 `Debug` 格式长期稳定

派生 `Debug` 的格式不是公共 API 契约。字段改名、结构调整都会影响输出。需要稳定文本
时应该实现 `Display` 或专门的序列化格式。

```rust
// Debug 适合临时排查
// 稳定导出格式应使用 Display、serde 或明确设计的 formatter
```

## 用 `Display` 输出用户可见文本

`Display` 面向用户，使用 `{}` 格式化。它通常需要手写，因为用户可见格式应由开发者
明确设计，而不是简单暴露内部字段结构。

### 最佳实践：为稳定的人类可读格式实现 `Display`

版本号、金额、错误摘要、用户可见 ID 等类型适合实现 `Display`。这样调用方可以通过
`format!("{value}")` 或 `value.to_string()` 得到稳定文本。

```rust
use std::fmt;

struct ReleaseVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl fmt::Display for ReleaseVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display 明确设计给用户看的版本号格式
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
```

### 最佳实践：实现 `Display` 后自动获得 `ToString`

标准库为所有实现 `Display` 的类型提供 `ToString`。因此通常不需要手动实现
`ToString`，只要实现 `Display` 即可。

```rust
use std::fmt;

struct UserId(u64);

impl fmt::Display for UserId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "user-{}", self.0)
    }
}

let text = UserId(42).to_string();
```

### 常见误区：不要用 `Display` 暴露调试细节

用户可见文本应该稳定、简洁、有业务含义。字段名、内部状态、缓存信息更适合放在
`Debug` 中，而不是 `Display`。

```rust
// Display 推荐："1.70.0"
// Debug 可以是："ReleaseVersion { major: 1, minor: 70, patch: 0 }"
```

### 常见误区：不要为同一类型塞入多个互相冲突的显示格式

一个类型只能有一个 `Display` 实现。如果同一数据需要短格式、长格式、带单位格式等多种
展示方式，应提供包装类型或显式方法，而不是让 `Display` 承担所有场景。

```rust
struct Bytes(u64);

// Bytes 的 Display 应选择一个最稳定、最常用的格式
// 其他格式可以通过 bytes.as_kib_text() 这类显式方法提供
```

## 其他标准 trait 速览

前面的十个 trait 是最常见、最基础的一组。下面这些标准 trait 同样重要，但它们往往
属于更具体的专题，适合先在这里建立地图，再到对应章节深入学习。

| trait | 表达的能力 | 常见场景 | 深入学习方向 |
| --- | --- | --- | --- |
| `From` / `Into` | 不会失败的类型转换 | ID 包装类型、配置类型、错误类型转换 | 错误处理、类型转换 |
| `TryFrom` / `TryInto` | 可能失败的类型转换 | 字符串解析为领域类型、范围校验转换 | 错误处理、类型转换 |
| `AsRef` / `AsMut` | 低成本引用转换 | API 同时接收 `String`、`&str`、`PathBuf`、`Path` | 泛型 API 设计 |
| `Borrow` / `BorrowMut` | 借用为等价视图 | `HashMap<String, V>` 用 `&str` 查询 | 集合、泛型 API 设计 |
| `Iterator` | 按序产生元素 | `map`、`filter`、`collect`、`sum` | Iterator 与闭包 |
| `IntoIterator` | 转换为迭代器 | `for` 循环、集合消费或借用迭代 | Iterator 与集合 |
| `Drop` | 离开作用域时清理资源 | 文件、连接、锁守卫、临时目录 | RAII、智能指针、资源管理 |
| `Deref` / `DerefMut` | 像引用一样访问内部值 | `Box<T>`、`Rc<T>`、`String` 到 `str` | 智能指针 |
| `Fn` / `FnMut` / `FnOnce` | 闭包捕获和调用能力 | 回调、函数参数、迭代器适配器 | 闭包 |
| `Error` | 标准错误类型接口 | 自定义错误、错误链、错误源 | 错误处理 |
| `Send` / `Sync` | 跨线程移动或共享的安全标记 | 线程、异步运行时、共享状态并发 | 并发编程 |
| `Read` / `Write` | 字节流读写 | 文件、网络、内存缓冲区 | I/O |
| `Add` / `Sub` / `Mul` / `Div` | 操作符重载 | 向量、矩阵、金额、时间跨度 | 高级 trait、领域建模 |
| `Index` / `IndexMut` | 下标访问 | 自定义集合、矩阵、网格 | 集合、操作符重载 |

这些 trait 的学习重点不是记住名字，而是建立一个判断习惯：当某个类型实现了某个标准
trait，它就在向调用方承诺一种稳定能力。实现 trait 前先确认语义是否成立；使用 trait
bound 前先确认函数体真正需要哪种能力。
