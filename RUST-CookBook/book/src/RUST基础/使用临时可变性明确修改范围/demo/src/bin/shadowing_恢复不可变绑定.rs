fn main() {
    let mut message = String::from("rust");
    message.make_ascii_uppercase();

    let message = message;
    assert_eq!(message, "RUST");
}
