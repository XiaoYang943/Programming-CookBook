fn double(value: Option<i32>) -> Option<i32> {
    let Some(value) = value else {
        return None;
    };
    Some(value * 2)
}

fn main() {
    assert_eq!(double(Some(21)), Some(42));
    assert_eq!(double(None), None);
}
