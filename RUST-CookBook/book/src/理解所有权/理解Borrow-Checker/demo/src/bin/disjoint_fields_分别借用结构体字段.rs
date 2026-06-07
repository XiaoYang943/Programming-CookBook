struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 1, y: 2 };
    let x = &mut point.x;
    let y = &mut point.y;

    *x += 10;
    *y += 20;
    assert_eq!((point.x, point.y), (11, 22));
}
