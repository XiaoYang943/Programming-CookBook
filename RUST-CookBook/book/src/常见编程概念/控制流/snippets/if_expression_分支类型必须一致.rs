fn main() {
    let condition = true;
    let value = if condition { 5 } else { "six" };
    assert_eq!(value, 5);
}
