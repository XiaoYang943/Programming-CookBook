fn length(value: &String) -> usize {
    value.len()    // 借来的值，默认不可修改、不可释放
}

fn main() {
    let text = String::from("Rust");

    assert_eq!(length(&text), 4); // 获取变量s的引用，借用给change方法使用
    assert_eq!(text, "Rust");
}
