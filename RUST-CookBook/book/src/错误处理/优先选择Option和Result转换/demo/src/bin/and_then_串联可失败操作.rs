fn reciprocal(value: i32) -> Option<f64> {
    (value != 0).then_some(1.0 / value as f64)
}

fn main() {
    assert_eq!("2".parse::<i32>().ok().and_then(reciprocal), Some(0.5));
}
