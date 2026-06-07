fn r#match(value: i32) -> bool {
    value == 42
}

fn main() {
    assert!(r#match(42));
}
