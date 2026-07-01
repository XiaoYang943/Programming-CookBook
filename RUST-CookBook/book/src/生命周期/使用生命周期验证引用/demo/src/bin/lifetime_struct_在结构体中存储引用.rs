struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let source = String::from("Rust ownership");
    let excerpt = Excerpt { text: &source[..4] };
    assert_eq!(excerpt.text, "Rust");
}
