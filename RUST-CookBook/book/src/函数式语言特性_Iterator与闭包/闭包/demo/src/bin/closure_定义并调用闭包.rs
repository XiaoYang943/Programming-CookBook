fn main() {
    let add_one = |value| value + 1;
    let calculate = |left: i32, right: i32| -> i32 { left * right };

    assert_eq!(add_one(4), 5);
    assert_eq!(calculate(3, 4), 12);
}
