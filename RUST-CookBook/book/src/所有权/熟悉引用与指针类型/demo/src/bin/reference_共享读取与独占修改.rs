fn main() {
    let mut value = 41;
    let shared = &value;
    assert_eq!(*shared, 41);

    let exclusive = &mut value;
    *exclusive += 1;
    assert_eq!(value, 42);
}
