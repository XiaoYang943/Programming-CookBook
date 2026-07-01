fn first_word(value: &str) -> &str {
    value.split_whitespace().next().unwrap_or("")
}

fn main() {
    assert_eq!(first_word("Rust Book"), "Rust");
}
