use std::convert::TryFrom;

fn main() {
    assert_eq!(u8::try_from(255_u16), Ok(255));
    assert!(u8::try_from(256_u16).is_err());
}
