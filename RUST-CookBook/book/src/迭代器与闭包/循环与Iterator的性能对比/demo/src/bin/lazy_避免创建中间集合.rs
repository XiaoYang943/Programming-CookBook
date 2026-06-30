fn main() {
    let result: i32 = (1..)
        .map(|value| value * value)
        .filter(|value| value % 2 == 0)
        .take(3)
        .sum();

    assert_eq!(result, 56);
}
