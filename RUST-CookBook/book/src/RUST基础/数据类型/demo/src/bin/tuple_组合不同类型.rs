fn main() {
    let user = ("Alice", 30, true);
    let (name, age, active) = user;

    assert_eq!(name, "Alice");
    assert_eq!(age, user.1);
    assert!(active);
}
