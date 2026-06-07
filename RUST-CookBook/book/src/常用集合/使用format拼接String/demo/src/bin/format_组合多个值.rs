fn main() {
    let language = String::from("Rust");
    let version = 2024;
    let text = format!("{language} edition {version}");
    assert_eq!(text, "Rust edition 2024");
    assert_eq!(language, "Rust");
}
