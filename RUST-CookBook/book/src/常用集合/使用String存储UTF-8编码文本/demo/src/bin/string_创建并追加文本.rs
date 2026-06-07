fn main() {
    let mut text = String::from("Rust");
    text.push(' ');
    text.push_str("语言");
    assert_eq!(text, "Rust 语言");
}
