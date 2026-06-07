use std::thread;

fn main() {
    let message = String::from("Rust");
    let worker_message = message.clone();

    let result = thread::spawn(move || worker_message).join().unwrap();

    assert_eq!(message, "Rust");
    assert_eq!(result, "Rust");
}
