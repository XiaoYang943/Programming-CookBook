fn length(value: &String) -> usize {
    value.len()
}

fn main() {
    let text = String::from("Rust");

    assert_eq!(length(&text), 4);
    assert_eq!(text, "Rust");
}
