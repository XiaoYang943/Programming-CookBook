fn fallback_from_error(error: &str) -> i32 {
    println!("处理错误: {error}");
    0
}

fn main() {
    let result: Result<i32, &str> = Ok(42);

    let value = result.unwrap_or_else(|error| {
        // `Ok(42)` 时，这个闭包不会执行。
        // 只有遇到 `Err(error)` 时，闭包才会拿到错误并生成备用值。
        fallback_from_error(error)
    });
    assert_eq!(value, 42);

    let result: Result<i32, &str> = Err("数字解析失败");

    let value = result.unwrap_or_else(|error| fallback_from_error(error));
    assert_eq!(value, 0);
}