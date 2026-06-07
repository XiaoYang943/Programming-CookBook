fn main() {
    let parsed = "42".parse::<i32>();
    let value = match parsed {
        Ok(value) => value,
        Err(_) => 0,
    };
    assert_eq!(value, 42);
}
