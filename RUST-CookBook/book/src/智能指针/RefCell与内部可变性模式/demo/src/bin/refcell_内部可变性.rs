use std::cell::RefCell;

fn main() {
    let value = RefCell::new(41);
    *value.borrow_mut() += 1;
    assert_eq!(*value.borrow(), 42);
}
