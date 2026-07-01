use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point(i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output { Point(self.0 + rhs.0) }
}

fn main() {
    assert_eq!(Point(1) + Point(2), Point(3));
}
