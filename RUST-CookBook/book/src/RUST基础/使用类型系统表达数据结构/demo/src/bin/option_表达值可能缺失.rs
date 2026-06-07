fn find_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn main() {
    assert_eq!(find_even(&[1, 3, 4]), Some(4));
    assert_eq!(find_even(&[1, 3, 5]), None);
}
