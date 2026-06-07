fn absolute(value: i32) -> i32 {
    if value < 0 {
        return -value;
    }

    value
}

fn main() {
    assert_eq!(absolute(-5), 5);
    assert_eq!(absolute(5), 5);
}
