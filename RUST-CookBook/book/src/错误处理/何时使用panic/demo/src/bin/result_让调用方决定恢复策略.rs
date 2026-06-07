fn parse_port(value: &str) -> Result<u16, std::num::ParseIntError> {
    value.parse()
}

fn main() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert!(parse_port("invalid").is_err());
}
