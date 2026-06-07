use std::rc::Rc;

fn main() {
    let first = Rc::new(String::from("Rust"));
    let second = Rc::clone(&first);
    assert_eq!(Rc::strong_count(&first), 2);
    assert_eq!(first.as_str(), second.as_str());
}
