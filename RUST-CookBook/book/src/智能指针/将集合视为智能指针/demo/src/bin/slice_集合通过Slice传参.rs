fn sum(values: &[i32]) -> i32 { values.iter().sum() }

fn main() {
    assert_eq!(sum(&vec![1, 2, 3]), 6);
    assert_eq!(sum(&[4, 5]), 9);
}
