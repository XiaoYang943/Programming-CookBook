fn main() {
    let mut text = String::from("Rust");
    text.push(' ');
    text.push_str("Book");
    assert_eq!(text, "Rust Book");
}
