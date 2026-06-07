fn main() {
    let value = 5;
    let value = value + 1;

    {
        let value = value * 2;
        assert_eq!(value, 12);
    }

    assert_eq!(value, 6);
}
