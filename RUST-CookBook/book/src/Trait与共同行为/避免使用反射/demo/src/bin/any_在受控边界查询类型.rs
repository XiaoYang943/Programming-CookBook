use std::any::Any;

fn is_string(value: &dyn Any) -> bool {
    value.is::<String>()
}

fn main() {
    assert!(is_string(&String::from("Rust")));
    assert!(!is_string(&42));
}
