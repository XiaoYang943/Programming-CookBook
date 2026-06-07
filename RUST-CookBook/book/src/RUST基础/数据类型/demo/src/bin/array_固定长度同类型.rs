fn main() {
    let months: [&str; 3] = ["January", "February", "March"];
    let zeros = [0; 3];

    assert_eq!(months.len(), 3);
    assert_eq!(months[1], "February");
    assert_eq!(zeros, [0, 0, 0]);
}
