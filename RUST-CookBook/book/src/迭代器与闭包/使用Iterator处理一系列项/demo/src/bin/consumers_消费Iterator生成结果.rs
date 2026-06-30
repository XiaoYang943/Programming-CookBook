fn main() {
    let values = [3, 7, 12, 18];

    assert_eq!(values.iter().sum::<i32>(), 40);
    assert_eq!(values.iter().find(|value| **value > 10), Some(&12));
    assert!(values.iter().any(|value| *value % 2 == 0));
    assert_eq!(
        values.iter().fold(1, |product, value| product * value),
        4536
    );
}
