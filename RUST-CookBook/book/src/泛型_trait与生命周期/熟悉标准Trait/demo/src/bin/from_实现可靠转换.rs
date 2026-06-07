struct UserId(u64);

impl From<u64> for UserId {
    fn from(value: u64) -> Self { Self(value) }
}

fn main() {
    let id: UserId = 42_u64.into();
    assert_eq!(id.0, 42);
}
