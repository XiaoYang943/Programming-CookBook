fn choose<'a>(first: &'a str, second: &'a str, use_first: bool) -> &'a str {
    if use_first { first } else { second }
}

fn main() {
    assert_eq!(choose("Rust", "Book", true), "Rust");
}
