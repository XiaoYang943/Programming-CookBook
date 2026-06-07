fn main() {
    let original = String::from("Rust");
    let moved = original;

    assert_eq!(moved, "Rust");
    assert_eq!(original, "Rust");
}
