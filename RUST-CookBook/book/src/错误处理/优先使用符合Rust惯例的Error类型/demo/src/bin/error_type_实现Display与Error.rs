use std::{error::Error, fmt};

#[derive(Debug)]
struct InvalidPort(u32);

impl fmt::Display for InvalidPort {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "端口 {} 超出范围", self.0)
    }
}

impl Error for InvalidPort {}

fn main() {
    assert_eq!(InvalidPort(70_000).to_string(), "端口 70000 超出范围");
}
