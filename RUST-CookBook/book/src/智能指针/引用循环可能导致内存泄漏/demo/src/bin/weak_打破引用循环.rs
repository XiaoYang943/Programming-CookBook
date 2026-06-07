use std::rc::Rc;

fn main() {
    let owner = Rc::new(42);
    let observer = Rc::downgrade(&owner);
    assert_eq!(observer.upgrade().as_deref(), Some(&42));
    drop(owner);
    assert!(observer.upgrade().is_none());
}
