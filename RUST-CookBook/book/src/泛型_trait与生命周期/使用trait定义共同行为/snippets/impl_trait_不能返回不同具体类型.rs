trait Value {}
impl Value for i32 {}
impl Value for String {}

fn create(flag: bool) -> impl Value {
    if flag { 42 } else { String::from("Rust") }
}

fn main() {
    let _value = create(true);
}
