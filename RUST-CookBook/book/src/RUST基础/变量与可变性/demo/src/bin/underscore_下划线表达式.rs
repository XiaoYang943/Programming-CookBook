#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let pair = (10, 20);
    let second: i32;

    // 丢弃元组中的第一个值，只给已有变量 second 赋值。
    (_, second) = pair;
    assert_eq!(second, 20);

    let x: i32;

    // 丢弃结构体中的 y，只给已有变量 x 赋值。
    Position { x, y: _ } = Position { x: 30, y: 40 };
    assert_eq!(x, 30);
}
