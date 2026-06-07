fn main() {
    let text = "中A";
    assert_eq!(text.chars().count(), 2);
    assert_eq!(text.bytes().count(), 4);
}
