fn starts_with_rust(value: &str) -> bool {
    value.starts_with("Rust")
}

fn main() {
    let owned = String::from("Rust ownership");

    assert!(starts_with_rust(&owned));
    assert!(starts_with_rust("Rust borrowing"));
}
