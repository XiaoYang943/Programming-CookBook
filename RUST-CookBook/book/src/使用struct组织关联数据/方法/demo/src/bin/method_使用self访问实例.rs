struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle { width: 3, height: 4 };
    assert_eq!(rectangle.area(), 12);
}
