fn main() {
    let values = [2, 4, 6, 8];
    let (sum, count) = values
        .iter()
        .fold((0, 0), |(sum, count), value| (sum + value, count + 1));

    assert_eq!(sum, 20);
    assert_eq!(count, 4);
    assert_eq!(sum / count, 5);
}
