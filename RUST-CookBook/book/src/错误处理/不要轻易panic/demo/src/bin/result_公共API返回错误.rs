fn element(values: &[i32], index: usize) -> Result<i32, &'static str> {
    values.get(index).copied().ok_or("索引越界")
}

fn main() {
    assert_eq!(element(&[1, 2], 1), Ok(2));
    assert!(element(&[1, 2], 9).is_err());
}
