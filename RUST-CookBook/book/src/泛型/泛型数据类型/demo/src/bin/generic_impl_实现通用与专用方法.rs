struct Point<T> {
    x: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

impl Point<f64> {
    fn is_positive(&self) -> bool { self.x > 0.0 }
}

fn main() {
    assert_eq!(*Point { x: 42 }.x(), 42);
    assert!(Point { x: 1.0 }.is_positive());
}
