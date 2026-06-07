fn main() {
    let number = 6;
    let parity = if number % 2 == 0 { "even" } else { "odd" };

    assert_eq!(parity, "even");
}
