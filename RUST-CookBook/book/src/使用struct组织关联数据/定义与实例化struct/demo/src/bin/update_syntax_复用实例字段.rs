struct User {
    name: String,
    active: bool,
}

fn main() {
    let first = User {
        name: String::from("Alice"),
        active: true,
    };
    let second = User {
        active: false,
        ..first
    };

    assert_eq!(second.name, "Alice");
    assert!(!second.active);
}
