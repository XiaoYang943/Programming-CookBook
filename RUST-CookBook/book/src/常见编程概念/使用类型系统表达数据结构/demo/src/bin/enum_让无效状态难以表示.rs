enum Connection {
    Disconnected,
    Connected { address: String },
}

fn address(connection: &Connection) -> Option<&str> {
    match connection {
        Connection::Disconnected => None,
        Connection::Connected { address } => Some(address),
    }
}

fn main() {
    let disconnected = Connection::Disconnected;
    let connected = Connection::Connected {
        address: "127.0.0.1".to_owned(),
    };

    assert_eq!(address(&disconnected), None);
    assert_eq!(address(&connected), Some("127.0.0.1"));
}
