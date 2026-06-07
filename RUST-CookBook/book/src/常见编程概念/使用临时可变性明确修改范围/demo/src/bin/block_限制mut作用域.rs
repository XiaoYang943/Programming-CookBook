fn main() {
    let values = {
        let mut values = Vec::new();
        values.push(1);
        values.push(2);
        values.push(3);
        values
    };

    assert_eq!(values, [1, 2, 3]);
}
