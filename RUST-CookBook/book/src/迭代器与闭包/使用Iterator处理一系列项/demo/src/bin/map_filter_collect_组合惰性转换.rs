fn main() {
    let values = [1, 2, 3, 4, 5];
    let doubled_even: Vec<i32> = values
        .iter()
        .filter(|value| **value % 2 == 0)
        .map(|value| value * 2)
        .collect();

    assert_eq!(doubled_even, [4, 8]);
}
