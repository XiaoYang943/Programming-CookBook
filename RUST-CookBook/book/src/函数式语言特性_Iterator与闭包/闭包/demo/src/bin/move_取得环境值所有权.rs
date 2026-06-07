fn main() {
    let message = String::from("owned by closure");
    let owns_message = move || message;

    assert_eq!(owns_message(), "owned by closure");
}
