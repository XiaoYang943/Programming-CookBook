struct User {
    name: String,
    active: bool,
}

fn main() {
    let first = User {
        name: String::from("Alice"),
        active: true,
    };
    let _second = User { ..first };
    assert_eq!(first.name, "Alice");
}
