fn main() {
    let identity = |value| value;
    let number = identity(5);
    let text = identity(String::from("rust"));

    assert_eq!(number, 5);
    assert_eq!(text, "rust");
}
