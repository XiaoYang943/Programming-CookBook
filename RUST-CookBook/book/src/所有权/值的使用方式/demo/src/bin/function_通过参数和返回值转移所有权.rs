// 将 text 作为参数传入时，String 的所有权【移动】到 add_suffix 的参数 value
// 原来的 text 随即失效，不能再被使用
fn add_suffix(mut value: String) -> String {
    value.push_str(" ownership");   // value 现在拥有这个 String，因此可以修改它

    value   // 返回 value 会把 String 的所有权【移出】函数，交给函数调用者
    // 因为所有权已经被移走，所以函数结束时不会释放这个 String
}

fn main() {
    let text = String::from("Rust");    // text 是这个 String 的初始所有者

    // add_suffix 返回 String 后，所有权又【移动】给返回值，并由新的 text 接收
    let text = add_suffix(text);

    assert_eq!(text, "Rust ownership");
    // main 结束时，最终所有者 text 离开作用域，String 在这里被释放
}
