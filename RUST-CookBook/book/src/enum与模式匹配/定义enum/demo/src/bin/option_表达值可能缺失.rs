fn find_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn main() {
    assert_eq!(find_even(&[1, 2, 3]), Some(2));
    assert_eq!(find_even(&[1, 3]), None);
}
