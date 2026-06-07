#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Complete,
}

fn main() {
    assert_eq!(Status::Pending, Status::Pending);
    assert_ne!(Status::Pending, Status::Complete);
}
