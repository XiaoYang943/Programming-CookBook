fn main() {
    let mut values = vec![1, 2, 3];

    {
        let first = &mut values[0];
        *first = 10;
    }

    values.push(4);
    assert_eq!(values, [10, 2, 3, 4]);
}
