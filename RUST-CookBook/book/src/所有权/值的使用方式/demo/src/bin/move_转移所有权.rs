fn main() {
    let original = String::from("Rust");

    let moved = original;

    assert_eq!(moved, "Rust");
    // assert_eq!(original, "Rust");    // 移动后继续使用旧变量会导致编译失败
}
