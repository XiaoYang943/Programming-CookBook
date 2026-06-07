fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let array = [1, 2, 3];
    let vector = vec![4, 5, 6];

    assert_eq!(sum(&array), 6);
    assert_eq!(sum(&vector), 15);
    assert_eq!(sum(&vector[1..]), 11);
}
