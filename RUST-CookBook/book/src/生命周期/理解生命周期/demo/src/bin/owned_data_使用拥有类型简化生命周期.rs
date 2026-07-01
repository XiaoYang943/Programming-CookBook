struct User {
    name: String,
}

fn main() {
    let source = String::from("Alice");
    let user = User { name: source };
    assert_eq!(user.name, "Alice");
}
