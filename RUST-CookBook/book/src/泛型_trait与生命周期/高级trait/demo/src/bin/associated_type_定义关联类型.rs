trait Container {
    type Item;
    fn item(&self) -> &Self::Item;
}

struct Boxed<T>(T);
impl<T> Container for Boxed<T> {
    type Item = T;
    fn item(&self) -> &T { &self.0 }
}

fn main() {
    assert_eq!(*Boxed(42).item(), 42);
}
