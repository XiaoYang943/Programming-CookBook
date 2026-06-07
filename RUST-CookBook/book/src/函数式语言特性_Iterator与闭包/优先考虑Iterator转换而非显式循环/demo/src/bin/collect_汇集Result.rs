fn parse_all(inputs: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs.iter().map(|text| text.parse()).collect()
}

fn main() {
    assert_eq!(parse_all(&["1", "2", "3"]).unwrap(), [1, 2, 3]);
    assert!(parse_all(&["1", "invalid", "3"]).is_err());
}
