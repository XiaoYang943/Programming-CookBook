fn main() {
    let mut original = String::from("Rust");

    let mut cloned = original.clone();

    original += " Hello";

    cloned += " 你好";

    assert_eq!(original, "Rust Hello");

    assert_eq!(cloned, "Rust 你好");
}
