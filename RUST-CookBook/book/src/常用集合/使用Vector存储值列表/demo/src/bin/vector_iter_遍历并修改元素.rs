fn main() {
    let mut values = vec![1, 2, 3];
    for value in &mut values {
        *value *= 2;
    }
    assert_eq!(values, [2, 4, 6]);
}
