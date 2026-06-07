fn main() {
    let prefix = Some(0);
    let values = [1, 2, 3];
    let combined: Vec<i32> = prefix.into_iter().chain(values).collect();

    assert_eq!(combined, [0, 1, 2, 3]);
}
