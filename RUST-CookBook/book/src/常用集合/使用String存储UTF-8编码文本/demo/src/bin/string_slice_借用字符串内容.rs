fn length(value: &str) -> usize {
    value.len()
}

fn main() {
    let text = String::from("Rust");
    assert_eq!(length(&text), 4);
}
