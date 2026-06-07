fn sum_of_squares_loop(values: &[u64]) -> u64 {
    let mut total = 0;
    for value in values {
        total += value * value;
    }
    total
}

fn sum_of_squares_iterator(values: &[u64]) -> u64 {
    values.iter().map(|value| value * value).sum()
}

fn main() {
    let values = [1, 2, 3, 4];
    assert_eq!(sum_of_squares_loop(&values), 30);
    assert_eq!(sum_of_squares_iterator(&values), 30);
}
