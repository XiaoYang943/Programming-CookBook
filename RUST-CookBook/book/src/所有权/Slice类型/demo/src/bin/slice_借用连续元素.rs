fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let values = [1, 2, 3, 4];

    assert_eq!(sum(&values[1..3]), 5);
}
