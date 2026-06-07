fn main() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2;
        }
    };

    assert_eq!(result, 10);
}
