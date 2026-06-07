struct User {
    name: String,
    active: bool,
}

fn main() {
    let mut user = User {
        name: String::from("Alice"),
        active: false,
    };
    user.active = true;

    assert_eq!(user.name, "Alice");
    assert!(user.active);
}
