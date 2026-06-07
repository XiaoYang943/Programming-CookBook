use std::{sync::Arc, thread};

fn main() {
    let value = Arc::new(String::from("Rust"));
    let worker_value = Arc::clone(&value);

    let length = thread::spawn(move || worker_value.len()).join().unwrap();

    assert_eq!(length, 4);
    assert_eq!(value.as_str(), "Rust");
}
