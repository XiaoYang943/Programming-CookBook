//! 演示模块级 Doc Comment。

/// 将输入值翻倍。
///
/// # Examples
///
/// ```
/// assert_eq!(2 * 2, 4);
/// ```
fn double(value: i32) -> i32 {
    value * 2
}

fn main() {
    assert_eq!(double(21), 42);
}
