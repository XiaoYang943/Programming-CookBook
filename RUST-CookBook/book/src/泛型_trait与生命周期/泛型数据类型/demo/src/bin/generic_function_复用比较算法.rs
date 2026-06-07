fn largest<T: PartialOrd>(values: &[T]) -> &T {
    let mut largest = &values[0];
    for value in &values[1..] {
        if value > largest {
            largest = value;
        }
    }
    largest
}

fn main() {
    assert_eq!(*largest(&[1, 3, 2]), 3);
    assert_eq!(*largest(&['a', 'z', 'b']), 'z');
}
