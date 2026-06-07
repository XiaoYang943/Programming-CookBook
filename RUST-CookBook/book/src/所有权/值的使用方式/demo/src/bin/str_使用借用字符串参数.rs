fn starts_with_rust(value: &str) -> bool {
    value.starts_with("Rust")
}

fn main() {
    let owned = String::from("Rust ownership");

    // &str 可以接收多种字符串来源
    assert!(starts_with_rust(&owned));                    // &String 自动转换为 &str
    assert!(starts_with_rust("Rust borrowing"));    // 字符串字面量
    assert!(starts_with_rust(&owned[0..2]));     // String 的一部分
}
