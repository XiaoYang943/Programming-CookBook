fn main() {
    let mut values = vec![1, 2, 3];
    assert_eq!(values.iter().sum::<i32>(), 6);

    values.iter_mut().for_each(|value| *value *= 2);
    assert_eq!(values, [2, 4, 6]);

    let owned: Vec<i32> = values.into_iter().map(|value| value + 1).collect();
    assert_eq!(owned, [3, 5, 7]);
}
