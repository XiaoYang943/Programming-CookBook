enum Shape { Square(u32), Rectangle(u32, u32) }

fn area(shape: Shape) -> u32 {
    match shape {
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    assert_eq!(area(Shape::Square(4)), 16);
    assert_eq!(area(Shape::Rectangle(3, 4)), 12);
}
