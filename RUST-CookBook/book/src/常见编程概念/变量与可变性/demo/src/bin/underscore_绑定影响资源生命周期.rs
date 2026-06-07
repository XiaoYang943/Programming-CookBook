use std::cell::Cell;

struct ResourceGuard<'a> {
    dropped: &'a Cell<bool>,
}

impl Drop for ResourceGuard<'_> {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

fn create_guard(dropped: &Cell<bool>) -> ResourceGuard<'_> {
    ResourceGuard { dropped }
}

fn main() {
    let discarded = Cell::new(false);
    _ = create_guard(&discarded);
    assert!(discarded.get());

    let ignored = Cell::new(false);
    let _ = create_guard(&ignored);
    assert!(ignored.get());

    let bound = Cell::new(false);
    {
        let _guard = create_guard(&bound);
        assert!(!bound.get());
    }
    assert!(bound.get());
}
