fn increment(mut value: i32) -> i32 {
    value += 1;
    value
}

fn main() {
    assert_eq!(increment(41), 42);
}
