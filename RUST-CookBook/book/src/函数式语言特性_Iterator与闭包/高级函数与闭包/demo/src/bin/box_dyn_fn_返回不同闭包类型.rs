fn operation(add: bool) -> Box<dyn Fn(i32) -> i32> {
    if add {
        Box::new(|value| value + 1)
    } else {
        Box::new(|value| value * 2)
    }
}

fn main() {
    assert_eq!(operation(true)(5), 6);
    assert_eq!(operation(false)(5), 10);
}
