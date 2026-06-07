fn create_value() -> String {
    String::new()
}

fn main() {
    _ = create_value(); // 下划线表达式：丢弃赋值位置中的值
    let _ = create_value(); // 通配符模式：明确忽略整个值
    let _unused = create_value(); // 真实绑定：保留值但不产生未使用警告
}
