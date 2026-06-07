fn main() {
    let mut original = 1;

    let mut copied = original;

    original += 100;

    assert_eq!(original, 101);

    assert_eq!(copied, 1);

    copied += 200;

    assert_eq!(copied, 201);
}
