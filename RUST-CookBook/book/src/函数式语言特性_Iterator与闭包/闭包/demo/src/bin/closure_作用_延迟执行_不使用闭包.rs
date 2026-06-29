fn fallback_from_error(error: &str) -> i32 {
    println!("处理错误: {error}");
    0
}

fn value_or_fallback(result: Result<i32, &str>) -> i32 {
    match result {
        Ok(value) => value,
        Err(error) => fallback_from_error(error),
    }
}

fn main() {
    let result: Result<i32, &str> = Ok(42);
    let value = value_or_fallback(result);
    assert_eq!(value, 42);

    let result: Result<i32, &str> = Err("数字解析失败");
    let value = value_or_fallback(result);
    assert_eq!(value, 0);
}