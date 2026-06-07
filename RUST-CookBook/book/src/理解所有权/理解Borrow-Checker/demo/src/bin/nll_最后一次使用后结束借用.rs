fn main() {
    let mut text = String::from("Rust");
    let shared = &text;
    assert_eq!(shared, "Rust");

    text.push_str(" ownership");
    assert_eq!(text, "Rust ownership");
}
