fn add_suffix(value: &mut String) {
    value.push_str(" ownership");
}

fn main() {
    let mut text = String::from("Rust");
    add_suffix(&mut text);

    assert_eq!(text, "Rust ownership");
}
