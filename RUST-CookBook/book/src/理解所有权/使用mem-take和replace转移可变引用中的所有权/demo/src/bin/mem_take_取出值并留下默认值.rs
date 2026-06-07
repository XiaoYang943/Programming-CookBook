use std::mem;

struct Message {
    body: String,
}

fn take_body(message: &mut Message) -> String {
    mem::take(&mut message.body)
}

fn main() {
    let mut message = Message {
        body: String::from("Rust"),
    };

    assert_eq!(take_body(&mut message), "Rust");
    assert!(message.body.is_empty());
}
