fn main() {
    let mut values = vec![1, 2, 3];
    let first = values.first().copied().unwrap();

    values.push(first);
    assert_eq!(values, [1, 2, 3, 1]);
}
