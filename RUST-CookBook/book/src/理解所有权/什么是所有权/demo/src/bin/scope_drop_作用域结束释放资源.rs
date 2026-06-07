use std::cell::Cell;

struct Resource<'a>(&'a Cell<bool>);

impl Drop for Resource<'_> {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

fn main() {
    let dropped = Cell::new(false);
    {
        let _resource = Resource(&dropped);
        assert!(!dropped.get());
    }
    assert!(dropped.get());
}
