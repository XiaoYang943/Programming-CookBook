struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    assert_eq!(area(&rectangle), 1_500);
}
