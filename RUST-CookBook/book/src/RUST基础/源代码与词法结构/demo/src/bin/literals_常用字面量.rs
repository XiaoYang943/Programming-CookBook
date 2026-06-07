fn main() {
    let decimal = 98_222;
    let hexadecimal = 0xff;
    let byte = b'A';
    let text = "Rust";
    let raw = r#"无需转义的 "Rust" 字符串"#;

    assert_eq!(decimal, 98_222);
    assert_eq!(hexadecimal, 255);
    assert_eq!(byte, 65);
    assert_eq!(text.len(), 4);
    assert!(raw.contains("\"Rust\""));
}
