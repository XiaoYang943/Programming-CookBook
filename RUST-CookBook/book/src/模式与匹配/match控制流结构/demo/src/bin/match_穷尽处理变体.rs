enum Direction { North, South }

fn code(direction: Direction) -> u8 {
    match direction {
        Direction::North => 1,
        Direction::South => 2,
    }
}

fn main() {
    assert_eq!(code(Direction::North), 1);
    assert_eq!(code(Direction::South), 2);
}
