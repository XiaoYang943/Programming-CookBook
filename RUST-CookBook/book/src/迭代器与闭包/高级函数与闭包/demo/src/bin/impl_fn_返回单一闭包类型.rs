fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |value| value * factor
}

fn main() {
    let double = multiplier(2);
    assert_eq!(double(6), 12);
}
