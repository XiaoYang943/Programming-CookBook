fn divide(left: i32, right: i32) -> Result<i32, &'static str> {
    if right == 0 { Err("除数不能为零") } else { Ok(left / right) }
}

fn main() {
    assert_eq!(divide(10, 2), Ok(5));
    assert!(divide(10, 0).is_err());
}
