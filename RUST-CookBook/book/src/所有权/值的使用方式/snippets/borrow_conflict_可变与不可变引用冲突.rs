fn main() {
    let mut text = String::from("Rust");

    let shared = &text;

    let exclusive = &mut text;

    assert_eq!(shared, "Rust");

    exclusive.push('!');
}
