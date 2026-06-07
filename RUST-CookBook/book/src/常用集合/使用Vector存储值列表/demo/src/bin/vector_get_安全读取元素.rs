fn main() {
    let values = vec![10, 20, 30];
    assert_eq!(values[1], 20);
    assert_eq!(values.get(9), None);
}
