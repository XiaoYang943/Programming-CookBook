struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let pair = Pair { first: 42, second: "Rust" };
    assert_eq!(pair.first, 42);
    assert_eq!(pair.second, "Rust");
}
