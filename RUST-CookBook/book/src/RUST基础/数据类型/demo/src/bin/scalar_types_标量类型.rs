fn main() {
    let signed: i32 = -42;
    let unsigned: u32 = 42;
    let decimal: f64 = 3.5;
    let enabled: bool = true;
    let symbol: char = '中';

    assert_eq!(signed.unsigned_abs(), unsigned);
    assert_eq!(decimal * 2.0, 7.0);
    assert!(enabled);
    assert_eq!(symbol.len_utf8(), 3);
}
