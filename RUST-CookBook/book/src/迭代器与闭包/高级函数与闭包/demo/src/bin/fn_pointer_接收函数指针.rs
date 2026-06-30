fn add_one(value: i32) -> i32 {
    value + 1
}

fn apply_twice(action: fn(i32) -> i32, value: i32) -> i32 {
    action(action(value))
}

fn main() {
    assert_eq!(apply_twice(add_one, 5), 7);
}
