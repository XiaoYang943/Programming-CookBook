fn main() {
    let mut values = vec![1, 2, 3];
    let first = &values[0];
    values.push(4);
    assert_eq!(*first, 1);
}
