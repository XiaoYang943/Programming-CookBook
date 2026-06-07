fn main() {
    assert_eq!(Some(21).map(|value| value * 2), Some(42));
    assert_eq!("21".parse::<i32>().map(|value| value * 2), Ok(42));
}
