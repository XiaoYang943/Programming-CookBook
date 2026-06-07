use std::cell::Cell;

struct Guard<'a>(&'a Cell<bool>);
impl Drop for Guard<'_> { fn drop(&mut self) { self.0.set(false); } }

fn main() {
    let locked = Cell::new(true);
    { let _guard = Guard(&locked); }
    assert!(!locked.get());
}
