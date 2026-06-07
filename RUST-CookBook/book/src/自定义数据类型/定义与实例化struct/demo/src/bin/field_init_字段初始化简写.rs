struct User {
    name: String,
    active: bool,
}

fn create_user(name: String) -> User {
    User { name, active: true }
}

fn main() {
    let user = create_user(String::from("Alice"));
    assert_eq!(user.name, "Alice");
    assert!(user.active);
}
