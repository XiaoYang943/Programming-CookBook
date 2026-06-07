#[derive(Debug, PartialEq)]
struct SendError {
    message: String,
}

fn send(message: String, connected: bool) -> Result<(), SendError> {
    if connected { Ok(()) } else { Err(SendError { message }) }
}

fn main() {
    let error = send(String::from("Rust"), false).unwrap_err();
    assert_eq!(error.message, "Rust");
}
