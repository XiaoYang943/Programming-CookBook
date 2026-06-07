fn main() {
    let original = String::from("Rust");
    let cloned = original.clone();

    assert_eq!(original, "Rust");
    assert_eq!(cloned, "Rust");
}
