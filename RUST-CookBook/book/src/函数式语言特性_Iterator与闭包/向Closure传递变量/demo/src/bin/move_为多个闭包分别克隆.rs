fn main() {
    let label = String::from("rust");

    let first_label = label.clone();
    let first = move || format!("{first_label}-one");

    let second_label = label.clone();
    let second = move || format!("{second_label}-two");

    assert_eq!(first(), "rust-one");
    assert_eq!(second(), "rust-two");
    assert_eq!(label, "rust");
}
