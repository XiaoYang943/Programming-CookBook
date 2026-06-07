fn main() {
    let text = String::from("hello world");
    let hello = &text[..5];
    let world = &text[6..];

    assert_eq!(hello, "hello");
    assert_eq!(world, "world");
}
