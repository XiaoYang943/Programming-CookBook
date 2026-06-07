use std::{cell::RefCell, rc::Rc};

fn main() {
    let value = Rc::new(RefCell::new(41));
    let shared = Rc::clone(&value);
    *shared.borrow_mut() += 1;
    assert_eq!(*value.borrow(), 42);
}
