fn choose_value(use_first: bool) -> i32 {
    let value: i32;

    if use_first {
        value = 10;
    }

    value
}

fn main() {
    assert_eq!(choose_value(true), 10);
}
