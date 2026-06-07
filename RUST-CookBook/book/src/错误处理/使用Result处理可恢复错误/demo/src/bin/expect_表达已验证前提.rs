fn main() {
    let port = "8080".parse::<u16>().expect("固定端口必须是有效 u16");
    assert_eq!(port, 8080);
}
