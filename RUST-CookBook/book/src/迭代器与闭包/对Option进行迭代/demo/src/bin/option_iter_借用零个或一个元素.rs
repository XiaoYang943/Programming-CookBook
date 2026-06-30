fn main() {
    let present = Some(String::from("rust"));
    let absent: Option<String> = None;

    assert_eq!(present.iter().map(String::len).collect::<Vec<_>>(), [4]);
    assert!(absent.iter().next().is_none());
    assert_eq!(present, Some(String::from("rust")));
}
