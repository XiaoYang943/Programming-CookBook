struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn from_tuple((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

fn main() {
    assert_eq!((Point::origin().x, Point::origin().y), (0, 0));
    let point = Point::from_tuple((1, 2));
    assert_eq!((point.x, point.y), (1, 2));
}
