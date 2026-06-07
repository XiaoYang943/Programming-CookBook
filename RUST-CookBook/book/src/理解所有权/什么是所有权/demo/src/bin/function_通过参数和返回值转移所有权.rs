fn add_suffix(mut value: String) -> String {
    value.push_str(" ownership");
    value
}

fn main() {
    let text = String::from("Rust");
    let text = add_suffix(text);

    assert_eq!(text, "Rust ownership");
}
