#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    assert_eq!(Point { x: 1, y: 2 }, Point { x: 1, y: 2 });
}
