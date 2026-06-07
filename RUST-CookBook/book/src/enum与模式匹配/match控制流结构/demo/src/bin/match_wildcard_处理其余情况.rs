fn category(value: u8) -> &'static str {
    match value {
        0 => "zero",
        1..=9 => "small",
        _ => "other",
    }
}

fn main() {
    assert_eq!(category(0), "zero");
    assert_eq!(category(42), "other");
}
