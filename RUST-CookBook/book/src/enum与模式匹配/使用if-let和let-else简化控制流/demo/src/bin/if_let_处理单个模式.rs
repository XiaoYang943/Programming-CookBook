fn main() {
    let value = Some(42);
    let mut observed = 0;
    if let Some(number) = value {
        observed = number;
    }
    assert_eq!(observed, 42);
}
