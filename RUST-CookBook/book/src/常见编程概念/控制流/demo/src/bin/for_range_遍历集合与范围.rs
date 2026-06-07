fn main() {
    let values = [10, 20, 30];
    let sum: i32 = values.into_iter().sum();
    let range_sum: i32 = (1..=3).sum();

    assert_eq!(sum, 60);
    assert_eq!(range_sum, 6);
}
