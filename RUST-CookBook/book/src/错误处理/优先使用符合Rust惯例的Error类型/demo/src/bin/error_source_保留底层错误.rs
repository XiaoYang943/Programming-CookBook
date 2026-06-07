use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug)]
struct PortError(ParseIntError);

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "无法解析端口")
    }
}

impl Error for PortError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

fn main() {
    let error = "invalid".parse::<u16>().map_err(PortError).unwrap_err();
    assert!(error.source().is_some());
}
