fn use_short<'a>(value: &'a str) -> &'a str { value }

fn main() {
    let static_value: &'static str = "Rust";
    let shorter = use_short(static_value);
    assert_eq!(shorter, "Rust");
}
