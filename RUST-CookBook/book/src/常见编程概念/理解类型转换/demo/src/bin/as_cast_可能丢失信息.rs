fn main() {
    let truncated = 300_u16 as u8;
    let precise = 42_u16 as u8;

    assert_eq!(truncated, 44);
    assert_eq!(precise, 42);
}
