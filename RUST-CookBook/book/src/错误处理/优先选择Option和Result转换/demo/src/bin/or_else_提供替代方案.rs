fn main() {
    let value = None.or_else(|| Some(42));
    assert_eq!(value, Some(42));
}
