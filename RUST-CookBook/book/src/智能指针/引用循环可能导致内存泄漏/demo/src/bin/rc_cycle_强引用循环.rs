use std::{cell::RefCell, rc::Rc};

struct Node {
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let first = Rc::new(Node { next: RefCell::new(None) });
    let second = Rc::new(Node { next: RefCell::new(Some(Rc::clone(&first))) });
    *first.next.borrow_mut() = Some(Rc::clone(&second));
    assert_eq!(Rc::strong_count(&first), 2);
}
