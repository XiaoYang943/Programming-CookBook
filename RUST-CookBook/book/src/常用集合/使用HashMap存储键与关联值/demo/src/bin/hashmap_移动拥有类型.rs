use std::collections::HashMap;

fn main() {
    let key = String::from("Blue");
    let value = String::from("10");
    let mut map = HashMap::new();
    map.insert(key, value);
    assert_eq!(map.get("Blue").map(String::as_str), Some("10"));
}
