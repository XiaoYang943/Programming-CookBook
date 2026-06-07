fn main() {
    let mut number = 3;
    let mut visited = Vec::new();

    while number > 0 {
        visited.push(number);
        number -= 1;
    }

    assert_eq!(visited, [3, 2, 1]);
}
