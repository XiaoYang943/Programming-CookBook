fn main() {
    let guess: u32 = "42".parse().expect("应当是有效数字");
    assert_eq!(guess, 42);
}
