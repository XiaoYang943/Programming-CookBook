#[derive(Debug, PartialEq)]
struct Millimeters(u32);

impl From<u32> for Millimeters {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

fn main() {
    let from = Millimeters::from(42);
    let into: Millimeters = 42_u32.into();

    assert_eq!(from, Millimeters(42));
    assert_eq!(into, Millimeters(42));
}
