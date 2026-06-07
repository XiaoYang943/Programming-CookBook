enum Message { Text(String), Quit }

fn length(message: Message) -> usize {
    match message {
        Message::Text(text) => text.len(),
        Message::Quit => 0,
    }
}

fn main() {
    assert_eq!(length(Message::Text(String::from("Rust"))), 4);
    assert_eq!(length(Message::Quit), 0);
}
