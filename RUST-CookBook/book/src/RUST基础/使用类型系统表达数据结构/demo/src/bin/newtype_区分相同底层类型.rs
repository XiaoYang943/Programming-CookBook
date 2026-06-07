#[derive(Debug, PartialEq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct OrderId(u64);

fn load_user(id: UserId) -> UserId {
    id
}

fn main() {
    assert_eq!(load_user(UserId(42)), UserId(42));
    assert_eq!(OrderId(42), OrderId(42));
}
