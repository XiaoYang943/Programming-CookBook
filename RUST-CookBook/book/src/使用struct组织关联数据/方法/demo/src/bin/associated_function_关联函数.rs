struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let point = Point::origin();
    assert_eq!((point.x, point.y), (0, 0));
}
