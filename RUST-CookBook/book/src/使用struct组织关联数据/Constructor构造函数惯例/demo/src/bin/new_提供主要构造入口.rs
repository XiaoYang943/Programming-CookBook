struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let point = Point::new(1, 2);
    assert_eq!((point.x, point.y), (1, 2));
}
