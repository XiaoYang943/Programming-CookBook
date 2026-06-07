fn main() {
    let reference;
    {
        let value = String::from("Rust");
        reference = &value;
    }
    assert_eq!(reference, "Rust");
}
