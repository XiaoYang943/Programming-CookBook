use std::cell::RefCell;

struct Item<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
impl Drop for Item<'_> { fn drop(&mut self) { self.1.borrow_mut().push(self.0); } }

fn main() {
    let order = RefCell::new(Vec::new());
    { let _first = Item("first", &order); let _second = Item("second", &order); }
    assert_eq!(*order.borrow(), ["second", "first"]);
}
