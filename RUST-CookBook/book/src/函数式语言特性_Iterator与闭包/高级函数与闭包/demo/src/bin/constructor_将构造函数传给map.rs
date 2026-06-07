#[derive(Debug, PartialEq)]
enum Status {
    Value(u32),
}

fn main() {
    let statuses: Vec<Status> = (1..=3).map(Status::Value).collect();

    assert_eq!(
        statuses,
        [Status::Value(1), Status::Value(2), Status::Value(3)]
    );
}
