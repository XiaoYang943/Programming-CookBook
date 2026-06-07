struct Message {
    body: String,
}

fn take_body(message: &mut Message) -> String {
    message.body
}

fn main() {
    let mut message = Message {
        body: String::from("Rust"),
    };
    let _body = take_body(&mut message);
}
