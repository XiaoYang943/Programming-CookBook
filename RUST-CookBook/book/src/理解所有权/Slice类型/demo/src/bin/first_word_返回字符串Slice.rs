fn first_word(value: &str) -> &str {
    value.split_whitespace().next().unwrap_or("")
}

fn main() {
    let text = String::from("Rust ownership");

    assert_eq!(first_word(&text), "Rust");
}
