fn dangling() -> &String {
    let value = String::from("Rust");
    &value
}

fn main() {
    let _value = dangling();
}
