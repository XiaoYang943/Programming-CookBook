fn main() {
    let optional_values = [Some(1), None, Some(3), None, Some(5)];
    let values: Vec<i32> = optional_values.into_iter().flatten().collect();

    assert_eq!(values, [1, 3, 5]);
}
