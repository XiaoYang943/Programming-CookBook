use std::cell::RefCell;

fn main() {
    let value = RefCell::new(42);
    let _shared = value.borrow();
    let _exclusive = value.borrow_mut();
}
