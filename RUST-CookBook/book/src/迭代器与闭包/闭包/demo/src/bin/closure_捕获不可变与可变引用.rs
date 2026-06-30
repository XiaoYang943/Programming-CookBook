fn main() {
    let values = vec![1, 2, 3];
    let contains_two = || values.contains(&2);
    assert!(contains_two());

    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    assert_eq!(count, 2);
}
