fn double(value: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(value.parse::<i32>()? * 2)
}

fn main() {
    assert_eq!(double("21"), Ok(42));
    assert!(double("Rust").is_err());
}
