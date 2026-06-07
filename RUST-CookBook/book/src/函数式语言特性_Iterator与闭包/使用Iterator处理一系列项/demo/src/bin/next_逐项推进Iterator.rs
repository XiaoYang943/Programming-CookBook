fn main() {
    let values = [10, 20];
    let mut iterator = values.iter();

    assert_eq!(iterator.next(), Some(&10));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), None);
}
