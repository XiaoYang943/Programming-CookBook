use std::cell::Cell;

struct Guard<'a>(&'a Cell<bool>);
impl Drop for Guard<'_> { fn drop(&mut self) { self.0.set(true); } }

fn main() {
    let dropped = Cell::new(false);
    { let _guard = Guard(&dropped); }
    assert!(dropped.get());
}
