enum Message {
    Quit,
    Text(String),
    Move { x: i32, y: i32 },
}

fn main() {
    let messages = [Message::Quit, Message::Text(String::from("Rust")), Message::Move { x: 1, y: 2 }];
    assert!(matches!(&messages[0], Message::Quit));
    assert!(matches!(&messages[1], Message::Text(text) if text == "Rust"));
    assert!(matches!(&messages[2], Message::Move { x: 1, y: 2 }));
}
